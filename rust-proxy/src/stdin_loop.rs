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
/// **Note:** The stdin read is blocking, so when the stop signal is set, the thread
/// will only exit after the next line is entered (or EOF is reached).
/// Users can press Enter or Ctrl+D to unblock and allow clean shutdown.
///
/// This thread will stop cleanly when:
/// - The stop signal is set AND user presses Enter
/// - The action loop channel is closed (action loop exited)
/// - EOF is reached (Ctrl+D on Unix, Ctrl+Z+Enter on Windows)
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
                    debug!("Stop signal received, exiting stdin loop");
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
                        // Check stop signal again after blocking read
                        if stop.load(std::sync::atomic::Ordering::Relaxed) {
                            debug!("Stop signal received after read, exiting stdin loop");
                            break;
                        }

                        // Successfully read a line
                        let line = buffer.trim_end().to_string();

                        // Only send non-empty lines
                        if !line.is_empty() {
                            debug!("Stdin received: {}", line);
                            // Check if the channel is still open
                            if tx.send(ActionMessage::StdinInput(line)).is_err() {
                                debug!("Action loop has stopped, exiting stdin loop");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        // Error reading from stdin
                        debug!("Error reading from stdin: {}", e);
                        std::thread::sleep(EMPTY_PAUSE);
                    }
                }

                // Check stop signal at end of loop
                if stop.load(std::sync::atomic::Ordering::Relaxed) {
                    debug!("Stop signal received at end of loop, exiting stdin loop");
                    break;
                }
            }
            debug!("Stdin reader stopped.");
        })
        .expect("Could not start stdin thread");
}
