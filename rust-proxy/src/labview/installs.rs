use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LabviewInstallError {
    #[cfg(target_os = "windows")]
    #[error("Cannot Access Windows Registry for Detection: {1}")]
    RegKeyError(#[source] std::io::Error, String),
    #[allow(unused)]
    #[error("Error Scanning LabVIEW Install Directory. Dir: \"{1}\"")]
    DirectoryError(#[source] std::io::Error, String),
}

/// Defines if LabVIEW is 64 bit or 32 bit.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Bitness {
    X86,
    X64,
}

impl fmt::Display for Bitness {
    /// Format the bitness to a human readable string.
    /// I'm not mad about this format, but consistent with the original code.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bitness::X64 => write!(f, "64bit"),
            Bitness::X86 => write!(f, "32bit"),
        }
    }
}

/// Represents a single install of LabVIEW.
#[derive(Clone, Debug, PartialEq)]
pub struct LabviewInstall {
    /// The install directory path.
    pub path: PathBuf,
    pub version: String,
    pub bitness: Bitness,
}

#[cfg(target_os = "windows")]
const LABVIEW_EXE: &str = "LabVIEW.exe";

#[cfg(not(target_os = "windows"))]
const LABVIEW_EXE: &str = "labview";

impl LabviewInstall {
    pub fn major_version(&self) -> String {
        // For current versions this just means taking everything before the space.
        // The unwrap is safe since even if there is no space, it will have a single return.
        self.version.split(' ').next().unwrap().to_owned()
    }

    /// Checks the path for links relative to the install.
    /// This is against vi.lib G CLI tools.
    /// This could be a future expansion point for things like <vi.lib>
    pub fn relative_path(&self, vi: &Path) -> PathBuf {
        let mut actual_path = self.path.clone();
        actual_path.push("vi.lib");
        actual_path.push("G CLI Tools");
        actual_path.push(vi);
        actual_path
    }

    /// Get the LabVIEW application path.
    pub fn application_path(&self) -> PathBuf {
        self.path.join(LABVIEW_EXE)
    }
}

/// Stores the full system installation details for LabVIEW.
pub struct SystemLabviewInstalls {
    // as BTree so we get sorting for free.
    versions: BTreeMap<(String, Bitness), LabviewInstall>,
}

impl SystemLabviewInstalls {
    /// Create a new instance to be populated.
    pub(in crate::labview) fn new() -> Self {
        Self {
            versions: BTreeMap::new(),
        }
    }

    /// Add the provided install to the system details.
    pub(in crate::labview) fn add_install(&mut self, install: LabviewInstall) {
        // Store with version minus SP1.
        let version = install.major_version();
        self.versions.insert((version, install.bitness), install);
    }

    /// Retrieve and installed version based on version and bitness.
    /// Matches to service packs of the same version.
    /// Returns None if nothing matches.
    pub fn get_version(&self, version: &str, bitness: Bitness) -> Option<&LabviewInstall> {
        self.versions.get(&(version.to_string(), bitness))
    }

    /// Provides a string output which can be printed to show the install details.
    /// Format based on original version.
    pub fn print_details(&self) -> String {
        let mut output = String::from("Detected LabVIEW versions:\n");

        for (_, install) in self.versions.iter() {
            // Note unwrap on path to string. Confident this wont panic since path is generated
            // by program so should be valid.
            output = output
                + &format!(
                    "{}, {} ({})\n",
                    install.version,
                    install.bitness,
                    install.path.to_str().unwrap()
                );
        }

        output
    }

    /// Get a default version which is just the latest.
    pub fn get_default(&self) -> Option<&LabviewInstall> {
        self.versions.values().last()
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
            path: PathBuf::from("C:\\LV2011"),
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
            path: PathBuf::from("C:\\LV2011_64"),
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
            path: PathBuf::from("C:\\LV2011_64"),
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
            path: PathBuf::from("C:\\LV2011_64"),
        };

        installs.add_install(install);

        // Non-existant version
        assert_eq!(installs.get_version("2012", Bitness::X64), None);
    }

    #[test]
    fn prints_details_in_compatible_format() {
        let mut installs = SystemLabviewInstalls::new();

        let install = LabviewInstall {
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
            path: PathBuf::from("C:\\LV2011_64"),
        };

        installs.add_install(install);

        let install = LabviewInstall {
            version: String::from("2012"),
            bitness: Bitness::X86,
            path: PathBuf::from("C:\\LV2012"),
        };

        installs.add_install(install);

        let printed = installs.print_details();

        let expected = "Detected LabVIEW versions:\n\
            2011 SP1, 64bit (C:\\LV2011_64)\n\
            2012, 32bit (C:\\LV2012)\n";

        assert_eq!(printed, expected);
    }

    #[test]
    fn get_short_version_from_install() {
        let install = LabviewInstall {
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
            path: PathBuf::from("C:\\LV2011_64"),
        };

        assert_eq!(install.major_version(), "2011")
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn properly_cased_app_path() {
        let install = LabviewInstall {
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
            path: PathBuf::from("C:\\LV2011_64"),
        };

        assert_eq!(
            install.application_path(),
            PathBuf::from("C:\\LV2011_64\\LabVIEW.exe")
        )
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn properly_cased_app_path() {
        let install = LabviewInstall {
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
            path: PathBuf::from("/LV2011_64"),
        };

        assert_eq!(
            install.application_path(),
            PathBuf::from("/LV2011_64/labview")
        )
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn get_install_relative_path() {
        let install = LabviewInstall {
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
            path: PathBuf::from("C:\\LV2011_64\\"),
        };

        let relative_path = install.relative_path(&PathBuf::from("test.vi"));

        assert_eq!(
            relative_path.to_str().unwrap(),
            "C:\\LV2011_64\\vi.lib\\G CLI Tools\\test.vi"
        );
    }
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn get_install_relative_path() {
        let install = LabviewInstall {
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
            path: PathBuf::from("/LV2011_64/"),
        };

        let relative_path = install.relative_path(&PathBuf::from("test.vi"));

        assert_eq!(
            relative_path.to_str().unwrap(),
            "/LV2011_64/vi.lib/G CLI Tools/test.vi"
        );
    }
}
