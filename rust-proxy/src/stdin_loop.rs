use std::io::{self, BufRead};
use std::sync::{
    Arc,
    atomic::AtomicBool,
    mpsc::{self, Receiver, RecvTimeoutError, Sender},
};
use std::time::Duration;

use log::{debug, error};

use crate::comms::{AppConnection, MessageToLV};

/// Interval at which buffered stdin data is sent to LabVIEW (in milliseconds).
/// This timeout triggers sending of accumulated input even without a newline.
/// Also used for checking the stop signal regularly for clean shutdown.
const STDIN_SEND_INTERVAL: Duration = Duration::from_millis(100);

/// Starts threads to read from stdin and send data to LabVIEW over the
/// dedicated signal connection (separate from the main comms channel).
///
/// This implementation uses a timeout-based approach where stdin input is sent
/// either when a newline is encountered OR after a timeout, whichever comes first.
///
/// **Benefits:**
/// - **Clean shutdown**: Stop signal checked every 100ms, exits automatically when LabVIEW sends EXIT
/// - **Flexible input**: Supports both line-based (interactive) and streaming (piped) input
/// - **No blocking on exit**: Unlike pure line-based readers, this won't hang waiting for Enter
/// - **Responsive**: Data sent immediately on newline, or within 100ms for partial input
///
/// **Use cases:**
/// - Interactive: `user types "add 5 3" + Enter` → sent immediately as "add 5 3\n"
/// - Piped: `echo -n "test" | g-cli app.vi` → sent after 100ms as "test"
/// - Streaming: `tail -f log | g-cli app.vi` → lines sent as they arrive
///
/// The threads will stop cleanly when:
/// - The stop signal is set (checked every 100ms)
/// - EOF is reached on stdin
/// - Writing to the signal connection fails (LabVIEW has gone away)
pub fn start(mut connection: AppConnection, stop: Arc<AtomicBool>) {
    // Create a channel for the stdin reader thread to send data
    let (stdin_tx, stdin_rx): (Sender<StdinEvent>, Receiver<StdinEvent>) = mpsc::channel();

    // Spawn stdin reader thread - reads lines and sends via channel
    let stop_clone = stop.clone();
    std::thread::Builder::new()
        .name("Stdin Reader".to_string())
        .spawn(move || {
            let stdin = io::stdin();
            let reader = stdin.lock();

            for line in reader.lines() {
                // Check if we should stop
                if stop_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    debug!("Stop signal detected in stdin reader");
                    break;
                }

                match line {
                    Ok(text) => {
                        if stdin_tx.send(StdinEvent::Line(text)).is_err() {
                            // Main loop has stopped
                            debug!("Stdin reader: main loop disconnected");
                            break;
                        }
                    }
                    Err(e) => {
                        debug!("Error reading from stdin: {}", e);
                        let _ = stdin_tx.send(StdinEvent::Error);
                        break;
                    }
                }
            }

            // Send EOF event
            let _ = stdin_tx.send(StdinEvent::Eof);
            debug!("Stdin reader thread stopped");
        })
        .expect("Could not start stdin reader thread");

    // Main stdin processing loop - handles timeout and stop signal
    std::thread::Builder::new()
        .name("Stdin Loop".to_string())
        .spawn(move || {
            let mut accumulated = String::new();
            let mut last_send_was_partial = false;

            loop {
                // Check if we should stop
                if stop.load(std::sync::atomic::Ordering::Relaxed) {
                    debug!("Stop signal received in stdin loop");
                    // Send any remaining accumulated data before exiting
                    if !accumulated.is_empty() {
                        send_to_labview(&mut connection, &accumulated, &stop);
                    }
                    break;
                }

                // Try to receive stdin data with timeout
                match stdin_rx.recv_timeout(STDIN_SEND_INTERVAL) {
                    Ok(StdinEvent::Line(line)) => {
                        // Got a complete line from stdin
                        accumulated.push_str(&line);
                        accumulated.push('\n'); // Preserve the newline

                        // Send immediately - this is a complete line
                        send_to_labview(&mut connection, &accumulated, &stop);
                        accumulated.clear();
                        last_send_was_partial = false;
                    }
                    Ok(StdinEvent::Eof) => {
                        // EOF reached - send any remaining data
                        debug!("EOF reached on stdin");
                        if !accumulated.is_empty() {
                            send_to_labview(&mut connection, &accumulated, &stop);
                        }
                        break;
                    }
                    Ok(StdinEvent::Error) => {
                        // Error from reader - send accumulated and exit
                        if !accumulated.is_empty() {
                            send_to_labview(&mut connection, &accumulated, &stop);
                        }
                        break;
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        // Timeout - send accumulated data if we have any
                        // This handles:
                        // 1. Piped input without newlines (echo -n "test")
                        // 2. Partial lines from slow typing
                        // 3. Streaming data that arrives in chunks
                        if !accumulated.is_empty() && !last_send_was_partial {
                            // Only send on first timeout to avoid spamming
                            // If user is typing slowly, we send what we have so far
                            debug!("Timeout - sending accumulated partial input");
                            send_to_labview(&mut connection, &accumulated, &stop);
                            accumulated.clear();
                            last_send_was_partial = true;
                        }
                        // Continue loop to check stop signal
                        // This is the key: every 100ms we check if LabVIEW sent EXIT
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        // Stdin reader died
                        debug!("Stdin reader disconnected");
                        if !accumulated.is_empty() {
                            send_to_labview(&mut connection, &accumulated, &stop);
                        }
                        break;
                    }
                }
            }

            debug!("Stdin loop stopped");
        })
        .expect("Could not start stdin loop thread");
}

/// Events from the stdin reader thread
enum StdinEvent {
    /// A complete line was read (without the newline)
    Line(String),
    /// EOF was reached
    Eof,
    /// An error occurred
    Error,
}

/// Sends accumulated stdin data to LabVIEW as an `STIN` message on the
/// dedicated signal connection.
fn send_to_labview(connection: &mut AppConnection, text: &str, stop: &Arc<AtomicBool>) {
    if text.is_empty() {
        return;
    }

    // Don't trim - preserve whitespace and newlines as they may be significant
    debug!("Sending to LabVIEW: {:?}", text);

    if let Err(e) = connection.write(MessageToLV::Stdin(text.to_string())) {
        error!("Failed to send stdin to LabVIEW: {}", e);
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
