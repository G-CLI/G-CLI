//! # LabVIEW Module
//!
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod installs;
pub mod process;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod install_detection_linux;
#[cfg(target_os = "windows")]
pub mod install_detection_win;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use install_detection_linux::*;
#[cfg(target_os = "windows")]
pub use install_detection_win::*;

use std::path::PathBuf;
use std::process::{Child, Command};

fn create_args(port: u16) -> Vec<String> {
    vec![String::from("--"), format!("-p:{}", port)]
}

pub fn launch_exe(path: PathBuf, port: u16) -> Result<Child, std::io::Error> {
    Command::new(path).args(create_args(port)).spawn()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_args_with_port() {
        let args = create_args(1234);

        let expected = vec![String::from("--"), String::from("-p:1234")];

        assert_eq!(args, expected);
    }
}
