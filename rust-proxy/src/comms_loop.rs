use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};
use std::time::Duration;

use log::debug;

use crate::{
    action_loop::ActionMessage,
    comms::{AppConnection, CommsError, MessageFromLV},
};

const EMPTY_PAUSE: Duration = Duration::from_millis(10);

/// Starts a thread which will monitor the incoming messages from LabVIEW.
///
/// The loop is driven by a non-blocking call to read from the connection.
/// We use non-blocking to allow checking of the stop signal.
///
/// This thread will panic if the action loop stops before this.
pub fn start(mut connection: AppConnection, tx: Sender<ActionMessage>, stop: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("Comms Loop".to_string())
        .spawn(move || {
            loop {
                match connection.read() {
                    Ok(message) => {
                        //before we send it - check if it is a message that terminates the connection.
                        let terminating_message = matches!(message, MessageFromLV::EXIT(_));

                        tx.send(ActionMessage::LVMessage(message))
                            .expect("Can't send to action loop.");

                        if terminating_message {
                            break;
                        }
                    }
                    Err(CommsError::ReadLvMessageError(e))
                        if e.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        //Limit the loop rate.
                        std::thread::sleep(EMPTY_PAUSE);
                    }
                    Err(error) => {
                        tx.send(ActionMessage::CommsError(Box::new(error)))
                            .expect("Cant send to action loop.");
                    }
                }
                //check stop outside of match to prevent an stream of messages holding the program open.
                if stop.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
            debug!("Comms reader stopped.");
        })
        .expect("Could not start comms thread");
}
