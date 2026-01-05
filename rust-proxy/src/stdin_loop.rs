use std::io::{self, BufRead};
use std::sync::{Arc, atomic::AtomicBool, mpsc::Sender};
use std::time::Duration;

use log::debug;

use crate::action_loop::ActionMessage;

const EMPTY_PAUSE: Duration = Duration::from_millis(10);

/// Starts a thread which will read from stdin and send commands to the action loop.
///
/// This thread reads line-by-line from stdin, allowing for interactive command input.
/// Each line is sent as a complete command to LabVIEW.
///
/// The loop is non-blocking and checks the stop signal periodically.
///
/// This thread will panic if the action loop stops before this.
pub fn start(tx: Sender<ActionMessage>, stop: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("Stdin Loop".to_string())
        .spawn(move || {
            let stdin = io::stdin();
            let mut reader = stdin.lock();
            let mut buffer = String::new();

            loop {
                // Check if we should stop
                if stop.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

                // Try to read a line from stdin
                buffer.clear();
                match reader.read_line(&mut buffer) {
                    Ok(0) => {
                        // EOF reached (Ctrl+D on Unix, Ctrl+Z on Windows)
                        debug!("Stdin EOF reached");
                        break;
                    }
                    Ok(_) => {
                        // Successfully read a line
                        let line = buffer.trim_end().to_string();

                        // Only send non-empty lines
                        if !line.is_empty() {
                            debug!("Stdin received: {}", line);
                            tx.send(ActionMessage::StdinInput(line))
                                .expect("Can't send to action loop.");
                        }
                    }
                    Err(e) => {
                        // Error reading from stdin
                        debug!("Error reading from stdin: {}", e);
                        std::thread::sleep(EMPTY_PAUSE);
                    }
                }
            }
            debug!("Stdin reader stopped.");
        })
        .expect("Could not start stdin thread");
}
