//! # LabVIEW Module
//!
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod installs;
pub mod process;
pub mod error;
mod port_discovery;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod install_detection_linux;
#[cfg(target_os = "windows")]
pub mod install_detection_win;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use install_detection_linux::*;
#[cfg(target_os = "windows")]
pub use install_detection_win::*;

use log::debug;
use std::path::PathBuf;

use port_discovery::Registration;

use self::error::LabVIEWError;

fn create_args(port: u16) -> Vec<String> {
    vec![String::from("--"), format!("-p:{}", port)]
}

pub fn launch_exe(path: PathBuf, port: u16) -> Result<process::MonitoredProcess, LabVIEWError> {
    process::MonitoredProcess::start(path, &create_args(port), None)
}

pub fn launch_lv(
    install: &installs::LabviewInstall,
    launch_vi: PathBuf,
    port: u16,
) -> Result<process::MonitoredProcess, LabVIEWError> {

    let mut vi = launch_vi.clone();

    if !vi.exists() {
        debug!("Looks like VI \"{}\" doesn't exist - Checking in vi.lib/G CLI Tools instead.", vi.to_string_lossy());
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
    let mut lv_args = vec![
        String::from("-unattended"),
        String::from(vi.to_str().unwrap()),
    ];
    lv_args.append(&mut create_args(port));

    let path = install.application_path();

    debug!("Launching: {:?} {}", path, lv_args.join(" "));

    process::MonitoredProcess::start(path, &lv_args, Some(registration))
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
