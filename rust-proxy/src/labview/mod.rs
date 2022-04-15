//! # LabVIEW Module
//!
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod error;
pub mod installs;
mod port_discovery;
pub mod process;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod install_detection_linux;
#[cfg(target_os = "windows")]
pub mod install_detection_win;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use install_detection_linux::*;
#[cfg(target_os = "windows")]
pub use install_detection_win::*;

use log::debug;
use std::{ffi::OsString, path::PathBuf};

use port_discovery::Registration;

use crate::os_string_support::join_os_string;

use self::error::LabVIEWError;

fn create_args(port: u16, allow_dialogs: bool) -> Vec<OsString> {
    let mut args = vec![];
    if !allow_dialogs {
        args.push(OsString::from("-unattended"));
    }

    args.push(OsString::from("--"));
    args.push(OsString::from(format!("-p:{}", port)));
    return args;
}

pub fn launch_exe(path: PathBuf, port: u16) -> Result<process::MonitoredProcess, LabVIEWError> {
    process::MonitoredProcess::start(path, &create_args(port, true), None)
}

pub fn launch_lv(
    install: &installs::LabviewInstall,
    launch_vi: PathBuf,
    port: u16,
    allow_dialogs: bool,
) -> Result<process::MonitoredProcess, LabVIEWError> {
    let mut vi = launch_vi.clone();

    if !vi.exists() {
        debug!(
            "Looks like VI \"{}\" doesn't exist - Checking in vi.lib/G CLI Tools instead.",
            vi.to_string_lossy()
        );
        let relative_path = install.relative_path(&vi);
        if relative_path.exists() {
            vi = relative_path;
        }
    }

    // Non-existant launch path
    if !vi.exists() {
        return Err(LabVIEWError::ViDoesNotExist(launch_vi));
    }

    let registration = Registration::register(&vi, install, &port)?;

    //todo: unwrap could fail here, can we validate it?
    let mut lv_args = vec![vi.as_os_str().to_owned()];
    lv_args.append(&mut create_args(port, allow_dialogs));

    let path = install.application_path();

    debug!(
        "Launching: {} {:?}",
        path.to_string_lossy(),
        join_os_string(&lv_args, " ")
    );

    process::MonitoredProcess::start(path, &lv_args, Some(registration))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_args_with_port() {
        let args = create_args(1234, false);

        let expected = vec![
            OsString::from("-unattended"),
            OsString::from("--"),
            OsString::from("-p:1234"),
        ];

        assert_eq!(args, expected);
    }

    #[test]
    fn test_args_no_dialog() {
        let args = create_args(1234, true);

        let expected = vec![OsString::from("--"), OsString::from("-p:1234")];

        assert_eq!(args, expected);
    }
}
