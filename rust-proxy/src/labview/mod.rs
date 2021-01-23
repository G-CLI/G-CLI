//! # LabVIEW Module
//!
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod process;

use std::collections::HashMap;
use std::path::PathBuf;

/// Defines if LabVIEW is 64 bit or 32 bit.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Bitness {
    X86,
    X64,
}

/// Represents a single install of LabVIEW.
#[derive(Clone, Debug, PartialEq)]
pub struct LabviewInstall {
    pub path: PathBuf,
    pub version: String,
    pub bitness: Bitness,
}

impl LabviewInstall {
    pub fn launch(&self) {
        //TBD: use the process sub module to launch from here.
    }
}

pub struct SystemLabviewInstalls {
    versions: HashMap<(Bitness, String), LabviewInstall>,
}

impl SystemLabviewInstalls {
    fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    fn add_install(&mut self, install: LabviewInstall) {
        let version = install.version.clone();

        self.versions.insert((install.bitness, version), install);
    }

    /// Retrieve and installed version based on version and bitness.
    /// Returns None if nothing matches.
    fn get_version(&self, version: &str, bitness: Bitness) -> Option<&LabviewInstall> {
        self.versions.get(&(bitness, version.to_string()))
    }
}

pub fn detect_installations() -> SystemLabviewInstalls {
    SystemLabviewInstalls::new()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn store_and_recover_32bit() {
        let mut installs = SystemLabviewInstalls::new();

        let install = LabviewInstall {
            version: String::from("2011"),
            bitness: Bitness::X86,
            path: PathBuf::from("C:\\LV2011\\labview.exe"),
        };

        installs.add_install(install.clone());

        assert_eq!(installs.get_version("2011", Bitness::X86), Some(&install));
    }

    #[test]
    fn store_and_recover_64bit() {
        let mut installs = SystemLabviewInstalls::new();

        let install = LabviewInstall {
            version: String::from("2011"),
            bitness: Bitness::X64,
            path: PathBuf::from("C:\\LV2011_64\\labview.exe"),
        };

        installs.add_install(install.clone());

        assert_eq!(installs.get_version("2011", Bitness::X64), Some(&install));
    }

    #[test]
    fn no_version_returns_none() {
        let mut installs = SystemLabviewInstalls::new();

        let install = LabviewInstall {
            version: String::from("2011"),
            bitness: Bitness::X64,
            path: PathBuf::from("C:\\LV2011_64\\labview.exe"),
        };

        installs.add_install(install);

        // Non-existant version
        assert_eq!(installs.get_version("2012", Bitness::X64), None);
    }
}
