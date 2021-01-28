//! Comms Module
//! ============
//!
//! Handles all TCP comms related functions.
//!
//!
use std::path::PathBuf;

/// All messages we can recieve from LabVIEW.
pub enum MessageFromLV {
    /// Exit with the exit code provided.
    EXIT(i32),
    /// Output to the command line.
    OUTP(String),
}

/// All messages we can send to LabVIEW
pub enum MessageToLV<'a> {
    /// Arguments sent as a tab delimited list
    ARGS(&'a [String]),
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
            MessageToLV::ARGS(args) => args.join("\t"),
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
        let args = [String::from("Test1")];

        let message = MessageToLV::ARGS(&args);

        let size = message.to_buffer(&mut buffer);

        let expected = "\x00\x00\x00\x09ARGSTest1";

        assert_eq!(size, 9 + 4); //9 plus the 4 for length.
        assert_eq!(&buffer[0..size], expected.as_bytes());
    }

    #[test]
    fn multiple_argument_message_to_buffer() {
        let mut buffer = [0u8; 9000];
        let args = [String::from("Test1"), String::from("Test2")];

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
}
