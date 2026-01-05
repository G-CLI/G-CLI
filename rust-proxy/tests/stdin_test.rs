// Integration test for stdin functionality
// This test doesn't require LabVIEW - it simulates the LabVIEW side

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

#[test]
fn test_stdin_message_format() {
    // This test verifies that a CMND message is correctly formatted

    // Start a mock LabVIEW server
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    // Spawn a thread to act as LabVIEW
    let server_thread = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0u8; 9000];

        // Read the message
        stream.read_exact(&mut buffer[0..4]).unwrap();
        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

        stream.read_exact(&mut buffer[4..(length as usize + 4)]).unwrap();

        // Extract message ID and content
        let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
        let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

        println!("Received message ID: {}", msg_id);
        println!("Received content: {}", content);

        // Verify it's a CMND message
        assert_eq!(msg_id, "CMND");
        assert_eq!(content, "test command");

        // Send back a success response (OUTP message)
        let response = "\x00\x00\x00\x0COUTPSuccess\n";
        stream.write_all(response.as_bytes()).unwrap();
    });

    // Give server time to start
    thread::sleep(Duration::from_millis(100));

    // Connect as client
    let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    // Send a CMND message
    let command = "test command";
    let msg_id = "CMND";
    let length = (msg_id.len() + command.len()) as u32;

    // Write message
    client.write_all(&length.to_be_bytes()).unwrap();
    client.write_all(msg_id.as_bytes()).unwrap();
    client.write_all(command.as_bytes()).unwrap();

    // Read response
    let mut buffer = [0u8; 9000];
    client.read_exact(&mut buffer[0..4]).unwrap();
    let resp_length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    client.read_exact(&mut buffer[4..(resp_length as usize + 4)]).unwrap();

    let resp_id = std::str::from_utf8(&buffer[4..8]).unwrap();
    let resp_content = std::str::from_utf8(&buffer[8..(resp_length as usize + 4)]).unwrap();

    assert_eq!(resp_id, "OUTP");
    assert_eq!(resp_content, "Success\n");

    server_thread.join().unwrap();
}

#[test]
fn test_multiple_stdin_commands() {
    // Test sending multiple commands in sequence

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let commands = vec!["add 5 3", "multiply 10 2", "exit"];
    let commands_clone = commands.clone();

    let server_thread = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0u8; 9000];

        for expected_cmd in commands_clone {
            // Read message
            stream.read_exact(&mut buffer[0..4]).unwrap();
            let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            stream.read_exact(&mut buffer[4..(length as usize + 4)]).unwrap();

            let msg_id = std::str::from_utf8(&buffer[4..8]).unwrap();
            let content = std::str::from_utf8(&buffer[8..(length as usize + 4)]).unwrap();

            assert_eq!(msg_id, "CMND");
            assert_eq!(content, expected_cmd);
            println!("✓ Received: {}", content);
        }
    });

    thread::sleep(Duration::from_millis(100));

    let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    for cmd in commands {
        let msg_id = "CMND";
        let length = (msg_id.len() + cmd.len()) as u32;

        client.write_all(&length.to_be_bytes()).unwrap();
        client.write_all(msg_id.as_bytes()).unwrap();
        client.write_all(cmd.as_bytes()).unwrap();
    }

    server_thread.join().unwrap();
}
