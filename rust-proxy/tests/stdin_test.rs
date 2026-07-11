// Integration test for stdin functionality
// This test doesn't require LabVIEW - it simulates the LabVIEW side of the
// dedicated signal connection that carries STIN messages.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

#[test]
fn test_stdin_message_format() {
    // This test verifies that an STIN message is correctly formatted on the
    // signal channel.

    // Start a mock LabVIEW server acting as the signal channel listener.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    // Spawn a thread to act as LabVIEW
    let server_thread = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0u8; 9000];

        // Read the message
        stream.read_exact(&mut buffer[0..4]).unwrap();
        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

        stream
            .read_exact(&mut buffer[4..(length as usize + 4)])
            .unwrap();

        // Extract message ID and content
        let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
        let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

        println!("Received message ID: {}", msg_id);
        println!("Received content: {}", content);

        // Verify it's an STIN message
        assert_eq!(msg_id, "STIN");
        assert_eq!(content, "test command\n");
    });

    // Give server time to start
    thread::sleep(Duration::from_millis(100));

    // Connect as client (simulating g-cli's signal connection)
    let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    // Send an STIN message
    let command = "test command\n";
    let msg_id = "STIN";
    let length = (msg_id.len() + command.len()) as u32;

    client.write_all(&length.to_be_bytes()).unwrap();
    client.write_all(msg_id.as_bytes()).unwrap();
    client.write_all(command.as_bytes()).unwrap();

    server_thread.join().unwrap();
}

#[test]
fn test_multiple_stdin_lines() {
    // Test sending multiple lines in sequence over the signal channel.

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let lines = vec!["add 5 3\n", "multiply 10 2\n", "exit\n"];
    let lines_clone = lines.clone();

    let server_thread = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0u8; 9000];

        for expected_line in lines_clone {
            // Read message
            stream.read_exact(&mut buffer[0..4]).unwrap();
            let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            stream
                .read_exact(&mut buffer[4..(length as usize + 4)])
                .unwrap();

            let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
            let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

            assert_eq!(msg_id, "STIN");
            assert_eq!(content, expected_line);
            println!("Received: {}", content);
        }
    });

    thread::sleep(Duration::from_millis(100));

    let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    for line in lines {
        let msg_id = "STIN";
        let length = (msg_id.len() + line.len()) as u32;

        client.write_all(&length.to_be_bytes()).unwrap();
        client.write_all(msg_id.as_bytes()).unwrap();
        client.write_all(line.as_bytes()).unwrap();
    }

    server_thread.join().unwrap();
}

#[test]
fn test_signal_message_format() {
    // Verifies the reserved SIGI (Ctrl+C, see issue #188) message format on
    // the signal channel. Not yet sent by g-cli, but the wire format is
    // locked in now so LabVIEW-side parsing can be written against it.

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server_thread = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0u8; 9000];

        stream.read_exact(&mut buffer[0..4]).unwrap();
        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        stream
            .read_exact(&mut buffer[4..(length as usize + 4)])
            .unwrap();

        let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
        assert_eq!(msg_id, "SIGI");
        assert_eq!(length, 4); // id only, empty payload.
    });

    thread::sleep(Duration::from_millis(100));

    let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let msg_id = "SIGI";
    let length = msg_id.len() as u32;

    client.write_all(&length.to_be_bytes()).unwrap();
    client.write_all(msg_id.as_bytes()).unwrap();

    server_thread.join().unwrap();
}
