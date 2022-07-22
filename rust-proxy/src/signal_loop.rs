use crate::action_loop::ActionMessage;
use eyre::{Context, Result};
use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::time::Duration;

/// Starts a thread that monitors a ctrlc or SIGINT event.
///
///
pub fn start(tx: Sender<ActionMessage>, stop: Arc<AtomicBool>) -> Result<()> {
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
