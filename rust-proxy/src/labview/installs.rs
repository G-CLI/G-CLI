use std::collections::HashMap;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LabviewInstallError {
    #[error("Cannot Access Windows Registry for Detection: {1}")]
    RegKeyError(#[source] std::io::Error, String)
}



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

/// Stores the full system installation details for LabVIEW.
pub struct SystemLabviewInstalls {
    versions: HashMap<(Bitness, String), LabviewInstall>,
}

impl SystemLabviewInstalls {

    /// Create a new instance to be populated.
    pub(in crate::labview) fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    /// Add the provided install to the system details.
    pub(in crate::labview) fn add_install(&mut self, install: LabviewInstall) {

        // Store with version minus SP1.
        // For current versions this just means taking everything before the space.
        // The unwrap is safe since even if there is no space, it will have a single return.
        let version = install.version.split(" ").nth(0).unwrap().to_owned();

        self.versions.insert((install.bitness, version), install);
    }

    /// Retrieve and installed version based on version and bitness.
    /// Matches to service packs of the same version.
    /// Returns None if nothing matches.
    pub fn get_version(&self, version: &str, bitness: Bitness) -> Option<&LabviewInstall> {
        self.versions.get(&(bitness, version.to_string()))
    }
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
    fn find_sp1_from_base_version() {
        let mut installs = SystemLabviewInstalls::new();

        let install = LabviewInstall {
            version: String::from("2011 SP1"),
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
