// Mock LabVIEW application for testing g-cli stdin functionality
// This simulates a LabVIEW VI that responds to CMND messages

use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;

fn main() {
    // Get port from command line (g-cli passes this)
    let args: Vec<String> = env::args().collect();

    println!("[Mock LabVIEW] Starting...");

    // In real g-cli, the port is passed via --ActiveXServer flag
    // For testing, we'll just connect to a hardcoded port
    // You would get this from args in production

    let port = if args.len() > 1 {
        args[1].parse::<u16>().expect("Invalid port")
    } else {
        println!("[Mock LabVIEW] Usage: mock_labview <port>");
        println!("[Mock LabVIEW] Using default port 3363");
        3363
    };

    // Connect to g-cli
    let mut stream = match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(s) => {
            println!("[Mock LabVIEW] Connected to g-cli on port {}", port);
            s
        }
        Err(e) => {
            eprintln!("[Mock LabVIEW] Failed to connect: {}", e);
            return;
        }
    };

    let mut buffer = [0u8; 9000];

    // Message loop
    loop {
        // Read message length
        if let Err(e) = stream.read_exact(&mut buffer[0..4]) {
            println!("[Mock LabVIEW] Connection closed: {}", e);
            break;
        }

        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

        // Read message ID and content
        if let Err(e) = stream.read_exact(&mut buffer[4..(length as usize + 4)]) {
            println!("[Mock LabVIEW] Error reading message: {}", e);
            break;
        }

        let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
        let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

        println!("[Mock LabVIEW] Received: {} -> {}", msg_id, content);

        match msg_id {
            "ARGS" => {
                // Initial arguments
                let args: Vec<&str> = content.split('\t').collect();
                println!("[Mock LabVIEW] Initial args: {:?}", args);
            }
            "CCWD" => {
                // Current working directory
                println!("[Mock LabVIEW] Working directory: {}", content);
            }
            "CMND" => {
                // Stdin command - this is what we're testing!
                let parts: Vec<&str> = content.split_whitespace().collect();

                if parts.is_empty() {
                    send_output(&mut stream, "Error: Empty command\n");
                    continue;
                }

                let command = parts[0];
                let args = &parts[1..];

                println!("[Mock LabVIEW] Command: {}, Args: {:?}", command, args);

                // Simulate command processing
                match command {
                    "add" => {
                        if args.len() == 2 {
                            if let (Ok(a), Ok(b)) = (args[0].parse::<i32>(), args[1].parse::<i32>()) {
                                let result = a + b;
                                send_output(&mut stream, &format!("Result: {}\n", result));
                            } else {
                                send_error(&mut stream, "Error: Invalid numbers\n");
                            }
                        } else {
                            send_error(&mut stream, "Error: add requires 2 arguments\n");
                        }
                    }
                    "multiply" => {
                        if args.len() == 2 {
                            if let (Ok(a), Ok(b)) = (args[0].parse::<i32>(), args[1].parse::<i32>()) {
                                let result = a * b;
                                send_output(&mut stream, &format!("Result: {}\n", result));
                            } else {
                                send_error(&mut stream, "Error: Invalid numbers\n");
                            }
                        } else {
                            send_error(&mut stream, "Error: multiply requires 2 arguments\n");
                        }
                    }
                    "help" => {
                        send_output(&mut stream, "Available commands:\n");
                        send_output(&mut stream, "  add <num1> <num2>      - Add two numbers\n");
                        send_output(&mut stream, "  multiply <num1> <num2> - Multiply two numbers\n");
                        send_output(&mut stream, "  help                   - Show this help\n");
                        send_output(&mut stream, "  exit                   - Exit the program\n");
                    }
                    "exit" => {
                        send_output(&mut stream, "Goodbye!\n");
                        send_exit(&mut stream, 0);
                        break;
                    }
                    _ => {
                        send_error(&mut stream, &format!("Error: Unknown command '{}'. Type 'help' for available commands.\n", command));
                    }
                }

                // Flush output
                send_flush(&mut stream);
            }
            _ => {
                println!("[Mock LabVIEW] Unknown message type: {}", msg_id);
            }
        }
    }

    println!("[Mock LabVIEW] Exiting");
}

fn send_output(stream: &mut TcpStream, text: &str) {
    send_message(stream, "OUTP", text);
}

fn send_error(stream: &mut TcpStream, text: &str) {
    send_message(stream, "SERR", text);
}

fn send_flush(stream: &mut TcpStream) {
    send_message(stream, "OFLS", "");
}

fn send_exit(stream: &mut TcpStream, code: i32) {
    send_message(stream, "EXIT", &code.to_string());
}

fn send_message(stream: &mut TcpStream, msg_id: &str, content: &str) {
    let length = (msg_id.len() + content.len()) as u32;

    stream.write_all(&length.to_be_bytes()).unwrap();
    stream.write_all(msg_id.as_bytes()).unwrap();
    stream.write_all(content.as_bytes()).unwrap();
    stream.flush().unwrap();
}
