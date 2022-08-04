//! Comms Module
//! ============
//!
//! Handles all TCP comms related functions.
//!
//!
use std::convert::TryInto;
use std::ffi::OsString;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use std::{path::PathBuf, str::Utf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommsError {
    #[error("The size parameter is not a valid size value.")]
    SizeParameterInvalid,
    #[error("Message ID Is Not Valid UTF8")]
    MessageIdNotValidUTF8(#[source] Utf8Error),
    #[error("Message contents cannot be read as UTF8")]
    MessageContentsNotValidUTF8(#[source] Utf8Error),
    #[error("Exit code string can't be parsed as an integer: \"{1}\"")]
    ExitCodeStringNotParsable(#[source] std::num::ParseIntError, String),
    #[error("Message ID is invalid \"{0}\"")]
    UnknownMessageId(String),
    #[error("Unexpected EOF Error: LabVIEW has probably closed the connection")]
    ConnectionClosedEof(#[source] std::io::Error),
    #[error("Connection Aborted: LabVIEW has probably crashed or failed to properly close the connection")]
    ConenctionClosedAborted(#[source] std::io::Error),
    #[error("IO Error While Listening for LabVIEW to Connect")]
    WaitOnConnectionIoError(#[source] std::io::Error),
    #[error("IO Error When Reading Messages From LabVIEW")]
    ReadLvMessageError(#[source] std::io::Error),
    #[error("IO Error When Writing Messages to LabVIEW")]
    WriteLvMessageError(#[source] std::io::Error),
    #[error("Timed out waiting for app to connect to g-cli")]
    WaitOnConnectionTimeOut,
    #[error("System error setting up app connection")]
    ErrorCreatingConnection(#[source] std::io::Error),
    #[error("System error setting up app listener")]
    ErrorCreatingListener(#[source] std::io::Error),
}

/// Provides the TCP Connection to the App
pub struct AppListener {
    listener: TcpListener,
}

impl AppListener {
    /// Create the listener and reserve the port.
    pub fn new() -> Result<Self, CommsError> {
        let listener = TcpListener::bind("127.0.0.1:5000")
            .map_err(|e| CommsError::ErrorCreatingListener(e))?;

        // So we can implement a timeout later.
        listener
            .set_nonblocking(true)
            .map_err(|e| CommsError::ErrorCreatingListener(e))?;

        Ok(Self { listener })
    }

    /// Get a Connection
    pub fn wait_on_app(&self, timeout: Duration) -> Result<AppConnection, CommsError> {
        // The standard networking library doesn't contain a timeout based TCP listener.
        // There maybe better methods than polling but this is where we can start.

        // timeout to ms then divided by the wait time.
        let wait_time = Duration::from_millis(10);
        let iterations = timeout.as_millis() / wait_time.as_millis();
        let mut count = 0;
        loop {
            match self.listener.accept() {
                Ok((stream, _addr)) => {
                    return AppConnection::new(stream);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    count = count + 1;

                    if count < iterations {
                        //retry
                        sleep(wait_time);
                    } else {
                        return Err(CommsError::WaitOnConnectionTimeOut);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(CommsError::WaitOnConnectionIoError(e));
                }
            }
        }
    }

    /// Get the port for the listener.
    pub fn port(&self) -> u16 {
        let address = self.listener.local_addr().unwrap();
        address.port()
    }
}

///The operating connection on the app.AppConnection
pub struct AppConnection {
    stream: TcpStream,
    buffer: [u8; 9000],
}

impl AppConnection {
    pub fn new(stream: TcpStream) -> Result<Self, CommsError> {
        stream
            .set_nonblocking(true)
            .map_err(|e| CommsError::ErrorCreatingConnection(e))?;
        stream
            .set_nodelay(true)
            .map_err(|e| CommsError::ErrorCreatingConnection(e))?;
        Ok(Self {
            stream,
            buffer: [0u8; 9000],
        })
    }

    pub fn write(&mut self, message: MessageToLV) -> Result<(), CommsError> {
        let size = message.to_buffer(&mut self.buffer);
        let result = self.stream.write(&self.buffer[0..size]);
        //match return type.
        result
            .map_err(|e| CommsError::WriteLvMessageError(e))
            .map(|_| ())
    }

    pub fn read(&mut self) -> Result<MessageFromLV, CommsError> {
        self.stream
            .read_exact(&mut self.buffer[0..4])
            .map_err(wrap_read_error)?;

        let size = u32::from_be_bytes(self.buffer[0..4].try_into().unwrap());

        self.stream
            .read_exact(&mut self.buffer[4..(size as usize) + 4])
            .map_err(wrap_read_error)?;

        MessageFromLV::from_buffer(&self.buffer)
    }
}

fn wrap_read_error(e: std::io::Error) -> CommsError {
    match e.kind() {
        std::io::ErrorKind::UnexpectedEof => CommsError::ConnectionClosedEof(e),
        std::io::ErrorKind::ConnectionAborted => CommsError::ConenctionClosedAborted(e),
        _ => CommsError::ReadLvMessageError(e),
    }
}

/// All messages we can recieve from LabVIEW.
#[derive(Clone, PartialEq, Debug)]
pub enum MessageFromLV {
    /// Exit with the exit code provided.
    EXIT(i32),
    /// Output to the command line.
    OUTP(String),
    /// Output to Standard Error
    SERR(String),
}

impl MessageFromLV {
    /// Get the message from the buffer.
    pub fn from_buffer(buffer: &[u8; 9000]) -> Result<MessageFromLV, CommsError> {
        let length = i32::from_be_bytes(
            buffer[0..4]
                .try_into()
                .map_err(|_| CommsError::SizeParameterInvalid)?,
        );

        let id =
            std::str::from_utf8(&buffer[4..8]).map_err(|e| CommsError::MessageIdNotValidUTF8(e))?;
        let data_end: usize = 8 + (length as usize) - 4; // 8 = offset, 4 = already used for id
        let contents = std::str::from_utf8(&buffer[8..data_end])
            .map_err(|e| CommsError::MessageContentsNotValidUTF8(e))?;

        match id {
            "EXIT" => {
                let code = contents.parse::<i32>().map_err(|e| {
                    CommsError::ExitCodeStringNotParsable(e, String::from(contents))
                })?;
                Ok(MessageFromLV::EXIT(code))
            }
            "OUTP" => Ok(MessageFromLV::OUTP(contents.to_string())),
            "SERR" => Ok(MessageFromLV::SERR(contents.to_string())),
            _ => Err(CommsError::UnknownMessageId(String::from(id))),
        }
    }
}

/// All messages we can send to LabVIEW
#[derive(Clone, PartialEq, Debug)]
pub enum MessageToLV<'a> {
    /// Arguments sent as a tab delimited list
    ARGS(&'a [OsString]),
    /// Current working directory as a path.
    CCWD(PathBuf),
}

impl<'a> MessageToLV<'a> {
    /// Populate the buffer with the message to send.
    /// Assumes a buffer of 9000 bytes.
    /// Returns the size of bytes to actually write.
    pub fn to_buffer(&'a self, buffer: &mut [u8; 9000]) -> usize {
        let message_id = match self {
            MessageToLV::ARGS(_) => "ARGS",
            MessageToLV::CCWD(_) => "CCWD",
        };

        let message_contents = match &self {
            MessageToLV::ARGS(args) => args
                .iter()
                .map(|s| s.to_str())
                .collect::<Option<Vec<&str>>>()
                .unwrap()
                .join("\t"),
            MessageToLV::CCWD(path) => path.to_str().unwrap().to_string(),
        };

        let length = message_contents.len() + message_id.len();

        // Write the sections out to the buffer.
        // Could be a key area to look at for efficiency.
        // Note Rust is UTF8 but LabVIEW is ASCII. Not sure how we should handle UTF chars here.
        // I think ignoring for now is the best bet.
        buffer[0..4].copy_from_slice(&(length as u32).to_be_bytes());
        buffer[4..8].copy_from_slice(message_id.as_bytes());
        buffer[8..(8 + message_contents.len())].copy_from_slice(message_contents.as_bytes());

        length + 4 //4 for length encoding.
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn single_argument_message_to_buffer() {
        let mut buffer = [0u8; 9000];
        let args = [OsString::from("Test1")];

        let message = MessageToLV::ARGS(&args);

        let size = message.to_buffer(&mut buffer);

        let expected = "\x00\x00\x00\x09ARGSTest1";

        assert_eq!(size, 9 + 4); //9 plus the 4 for length.
        assert_eq!(&buffer[0..size], expected.as_bytes());
    }

    #[test]
    fn multiple_argument_message_to_buffer() {
        let mut buffer = [0u8; 9000];
        let args = [OsString::from("Test1"), OsString::from("Test2")];

        let message = MessageToLV::ARGS(&args);

        let size = message.to_buffer(&mut buffer);

        let expected = "\x00\x00\x00\x0FARGSTest1\tTest2";

        assert_eq!(size, 15 + 4); //9 plus the 4 for length.
        assert_eq!(&buffer[0..size], expected.as_bytes());
    }

    #[test]
    fn working_directory_to_buffer() {
        let mut buffer = [0u8; 9000];
        let dir = PathBuf::from("C:\\test");

        let message = MessageToLV::CCWD(dir);

        let size = message.to_buffer(&mut buffer);

        let expected = "\x00\x00\x00\x0BCCWDC:\\test";

        assert_eq!(size, 11 + 4); //9 plus the 4 for length.
        assert_eq!(&buffer[0..size], expected.as_bytes());
    }

    #[test]
    fn exit_command_from_buffer() {
        let mut buffer = [0u8; 9000];

        let input = "\x00\x00\x00\x07EXIT123";

        buffer[0..input.len()].copy_from_slice(input.as_bytes());

        let message = MessageFromLV::from_buffer(&buffer);

        assert_eq!(message.unwrap(), MessageFromLV::EXIT(123));
    }

    #[test]
    fn exit_command_invalid_string_from_buffer() {
        let mut buffer = [0u8; 9000];

        let input = "\x00\x00\x00\x07EXIT1.3";

        buffer[0..input.len()].copy_from_slice(input.as_bytes());

        let message = MessageFromLV::from_buffer(&buffer);

        match message {
            Ok(_) => panic!("Fail"),
            Err(CommsError::ExitCodeStringNotParsable(_, string)) => {
                assert_eq!(string, String::from("1.3"))
            }
            Err(_) => panic!("Fail"),
        }
    }

    #[test]
    fn error_on_unknown_id() {
        let mut buffer = [0u8; 9000];

        let input = "\x00\x00\x00\x07EXTT123";

        buffer[0..input.len()].copy_from_slice(input.as_bytes());

        let message = MessageFromLV::from_buffer(&buffer);

        match message {
            Err(CommsError::UnknownMessageId(id)) => assert_eq!(id, String::from("EXTT")),
            _ => panic!("Not ID Error"),
        }
    }

    #[test]
    fn output_from_buffer() {
        let mut buffer = [0u8; 9000];

        let input = "\x00\x00\x00\x11OUTPHello, World\n";

        buffer[0..input.len()].copy_from_slice(input.as_bytes());

        let message = MessageFromLV::from_buffer(&buffer);

        assert_eq!(
            message.unwrap(),
            MessageFromLV::OUTP(String::from("Hello, World\n"))
        );
    }

    #[test]
    fn error_output_from_buffer() {
        let mut buffer = [0u8; 9000];

        let input = "\x00\x00\x00\x11SERRHello, World\n";

        buffer[0..input.len()].copy_from_slice(input.as_bytes());

        let message = MessageFromLV::from_buffer(&buffer);

        assert_eq!(
            message.unwrap(),
            MessageFromLV::SERR(String::from("Hello, World\n"))
        );
    }
}
