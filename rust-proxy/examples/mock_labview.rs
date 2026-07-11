// Mock LabVIEW application for testing g-cli stdin functionality
// This simulates a LabVIEW VI that connects to both the main comms channel
// and the dedicated signal channel, and responds to STIN messages.
//
// g-cli launches this with `-- -p:<main port> -p2:<signal port>`, mirroring
// what it passes to a real LabVIEW executable.

use std::env;
use std::io::Read;
use std::net::TcpStream;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("[Mock LabVIEW] Starting...");

    let main_port = find_port(&args, "-p:").unwrap_or_else(|| {
        println!("[Mock LabVIEW] Usage: mock_labview -- -p:<port> -p2:<signal port>");
        println!("[Mock LabVIEW] Using default main port 3363");
        3363
    });
    let signal_port = find_port(&args, "-p2:").unwrap_or_else(|| {
        println!("[Mock LabVIEW] Using default signal port 3364");
        3364
    });

    let mut main_stream = match TcpStream::connect(format!("127.0.0.1:{}", main_port)) {
        Ok(s) => {
            println!("[Mock LabVIEW] Connected main channel on port {}", main_port);
            s
        }
        Err(e) => {
            eprintln!("[Mock LabVIEW] Failed to connect main channel: {}", e);
            return;
        }
    };

    let signal_stream = match TcpStream::connect(format!("127.0.0.1:{}", signal_port)) {
        Ok(s) => {
            println!(
                "[Mock LabVIEW] Connected signal channel on port {}",
                signal_port
            );
            s
        }
        Err(e) => {
            eprintln!("[Mock LabVIEW] Failed to connect signal channel: {}", e);
            return;
        }
    };

    // Signal channel only ever carries STIN/SIGI from g-cli, so it gets its
    // own thread and never needs to write anything back.
    let signal_thread = thread::spawn(move || read_signal_channel(signal_stream));

    // Main channel loop - reads ARGS/CCWD, then EXIT/OFLS as they occur.
    let mut buffer = [0u8; 9000];
    loop {
        if let Err(e) = main_stream.read_exact(&mut buffer[0..4]) {
            println!("[Mock LabVIEW] Main channel closed: {}", e);
            break;
        }

        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

        if let Err(e) = main_stream.read_exact(&mut buffer[4..(length as usize + 4)]) {
            println!("[Mock LabVIEW] Error reading main channel message: {}", e);
            break;
        }

        let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
        let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

        println!("[Mock LabVIEW] Main channel received: {} -> {}", msg_id, content);

        match msg_id {
            "ARGS" => {
                let args: Vec<&str> = content.split('\t').collect();
                println!("[Mock LabVIEW] Initial args: {:?}", args);
            }
            "CCWD" => {
                println!("[Mock LabVIEW] Working directory: {}", content);
            }
            _ => {
                println!("[Mock LabVIEW] Unhandled main channel message type: {}", msg_id);
            }
        }
    }

    let _ = signal_thread.join();
    println!("[Mock LabVIEW] Exiting");
}

/// Reads STIN (and reserved SIGI) messages from the dedicated signal
/// connection until it closes.
fn read_signal_channel(mut stream: TcpStream) {
    let mut buffer = [0u8; 9000];
    loop {
        if let Err(e) = stream.read_exact(&mut buffer[0..4]) {
            println!("[Mock LabVIEW] Signal channel closed: {}", e);
            break;
        }

        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

        if let Err(e) = stream.read_exact(&mut buffer[4..(length as usize + 4)]) {
            println!("[Mock LabVIEW] Error reading signal channel message: {}", e);
            break;
        }

        let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
        let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

        match msg_id {
            "STIN" => println!("[Mock LabVIEW] stdin line: {:?}", content),
            "SIGI" => println!("[Mock LabVIEW] Ctrl+C signal received"),
            _ => println!("[Mock LabVIEW] Unknown signal channel message type: {}", msg_id),
        }
    }
}

/// Finds a `-p:<port>`/`-p2:<port>` style argument and parses the port out of it.
fn find_port(args: &[String], prefix: &str) -> Option<u16> {
    args.iter()
        .find_map(|arg| arg.strip_prefix(prefix))
        .and_then(|port_str| port_str.parse::<u16>().ok())
}
