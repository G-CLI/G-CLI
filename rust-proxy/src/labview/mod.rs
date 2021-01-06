//! # LabVIEW Module
//! 
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod process;

use std::path::PathBuf;

/// Defines if LabVIEW is 64 bit or 32 bit.
pub enum Bitness {
    x86,
    x64
}

/// Represents a single install of LabVIEW.
pub struct LabviewInstall {
    pub path: PathBuf,
    pub version: String,
    pub bitness: Bitness
}

impl LabviewInstall {
    pub fn launch(&self) {
        //TBD: use the process sub module to launch from here.
    }
}