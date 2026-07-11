use crate::action_loop::ActionMessage;
use crate::comms::{AppConnection, MessageToLV};
use eyre::{Context, Result};
use log::{debug, error};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{RecvTimeoutError, Sender, channel};
use std::time::Duration;

/// Starts a thread that monitors a ctrlc or SIGINT event.
///
/// `signal_connection` is a dedicated (cloned) handle to the signal channel,
/// used only to notify LabVIEW of the Ctrl+C via `SIGI` (issue #188) before
/// g-cli proceeds with its own shutdown. `None` when the signal channel is
/// disabled (`--no-signal`) - in that case LabVIEW just never finds out and
/// g-cli's own shutdown proceeds as before.
pub fn start(
    tx: Sender<ActionMessage>,
    stop: Arc<AtomicBool>,
    mut signal_connection: Option<AppConnection>,
) -> Result<()> {
    //We will run a local thread to wait on a signal from the handler
    //but also periodically check the stop.

    let (local_tx, local_rx) = channel();

    ctrlc::set_handler(move || {
        local_tx.send(()).expect("Send failed in handler");
    })
    .wrap_err("Should never set handler twice")?;

    std::thread::Builder::new()
        .name("Ctrl C Handler Thread".to_string())
        .spawn(move || {
            loop {
                match local_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(_) => {
                        if let Some(connection) = &mut signal_connection {
                            debug!("Sending Ctrl+C signal to LabVIEW");
                            if let Err(e) = connection.write(MessageToLV::Signal) {
                                error!("Failed to send Ctrl+C signal to LabVIEW: {}", e);
                            }
                        }
                        tx.send(ActionMessage::CtrlC).expect("Action loop gone?");
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        //no message - just check stop.
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        panic!("This should never be disconnected.");
                    }
                }
            }
            debug!("Signal Handler Stopped.");
        })?;

    Ok(())
}
