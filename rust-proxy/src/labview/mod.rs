//! # LabVIEW Module
//!
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod installs;
pub mod process;

#[cfg(target_os = "linux")]
pub mod install_detection_linux;
#[cfg(target_os = "windows")]
pub mod install_detection_win;

#[cfg(target_os = "linux")]
pub use install_detection_linux::*;
#[cfg(target_os = "windows")]
pub use install_detection_win::*;
