use crate::comms::MessageFromLV;
use log::{debug, error};
use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};

pub enum ActionMessage {
    LVMessage(MessageFromLV),
    CommsError(Box<dyn Error + Send + Sync>),
    CtrlC,
}

pub enum ExitAction {
    /// Exit cleanly with the provided error code.
    CleanExit(i32),
    /// Kill signals have been recieved. Stop and kill all processes ASAP.
    ForcedExit,
}

pub struct ActionLoop {
    tx: Sender<ActionMessage>,
    rx: Receiver<ActionMessage>,
    stopped: Arc<AtomicBool>,
}

impl ActionLoop {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let stopped = Arc::new(AtomicBool::new(false));
        Self { tx, rx, stopped }
    }

    /// Get a copy of the channel to send messages to the action loop.
    pub fn get_channel(&self) -> Sender<ActionMessage> {
        self.tx.clone()
    }

    /// Get a copy of the stop signal used to monitor when the loop has stopped.
    pub fn get_stop_signal(&self) -> Arc<AtomicBool> {
        self.stopped.clone()
    }

    /// Consumes itself and runs the loop.
    ///
    /// Stops running once all writers drop their sender.
    /// returns an exit code to use.
    pub fn run(self) -> ExitAction {
        let Self { tx, rx, stopped } = self;

        let mut exit_action = ExitAction::CleanExit(0);

        //Force drop our own unused sender.
        drop(tx);

        //this will run until there are no senders.
        // In here we set the stop signal to notify senders to stop.
        // Then this will stop automatically once they are all gone allowing us to process further messages.
        for message in rx {
            match message {
                ActionMessage::LVMessage(MessageFromLV::OUTP(string)) => {
                    print!("{}", string);
                }
                ActionMessage::LVMessage(MessageFromLV::SERR(string)) => {
                    eprint!("{}", string);
                }
                ActionMessage::LVMessage(MessageFromLV::EXIT(code)) => {
                    exit_action = ExitAction::CleanExit(code);
                    set_stop(&stopped);
                }
                ActionMessage::CommsError(e) => {
                    exit_action = ExitAction::CleanExit(-1);
                    set_stop(&stopped);
                    error!("Comms Error: {}", e);
                }
                ActionMessage::CtrlC => {
                    set_stop(&stopped);
                    debug!("Recieved Ctrl+C Kill Signal");
                    exit_action = ExitAction::ForcedExit;
                }
            }
        }

        debug!("Action loop stopped.");
        exit_action
    }
}

//helper function to simplify multiple calls.
fn set_stop(stopped: &Arc<AtomicBool>) {
    stopped.store(true, std::sync::atomic::Ordering::Relaxed)
}
