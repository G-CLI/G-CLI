//! # LabVIEW Module
//!
//! `labview` contains functionality for finding, launching
//! and monitoring the labview process.

pub mod process;

use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

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
    fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    /// Add the provided install to the system details.
    fn add_install(&mut self, install: LabviewInstall) {

        // Store with version minus SP1.
        // For current versions this just means taking everything before the space.
        // The unwrap is safe since even if there is no space, it will have a single return.
        let version = install.version.split(" ").nth(0).unwrap().to_owned();

        self.versions.insert((install.bitness, version), install);
    }

    /// Retrieve and installed version based on version and bitness.
    /// Matches to service packs of the same version.
    /// Returns None if nothing matches.
    fn get_version(&self, version: &str, bitness: Bitness) -> Option<&LabviewInstall> {
        self.versions.get(&(bitness, version.to_string()))
    }
}

/// Scan the system for LabVIEW installs - Linux
pub fn detect_installations() -> Result<SystemLabviewInstalls, LabviewInstallError> {
    Ok(SystemLabviewInstalls::new())
}


/// Scan the system for LabVIEW installs and return their details.
#[cfg(target_os = "windows")]
pub fn detect_installations() -> Result<SystemLabviewInstalls, LabviewInstallError> {
    let mut system = SystemLabviewInstalls::new();

    // The key structure here is only valid for 64 bit OS.
    // I think this is acceptible - I don't expect support for 32 bit OS to be a critical starting point.

    const BASE_KEY: &str = "SOFTWARE\\National Instruments\\LabVIEW";
    const BASE_KEY_WOW: &str = "SOFTWARE\\WOW6432Node\\National Instruments\\LabVIEW";

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let labview_32_key = hklm.open_subkey(BASE_KEY_WOW).map_err(|err| LabviewInstallError::RegKeyError(err, BASE_KEY_WOW.to_owned()))?;
    let labview_64_key = hklm.open_subkey(BASE_KEY).map_err(|err| LabviewInstallError::RegKeyError(err, BASE_KEY.to_owned()))?;

    installations_from_labview_registry(labview_32_key, Bitness::X86, &mut system)?;
    installations_from_labview_registry(labview_64_key, Bitness::X64, &mut system)?;
    Ok(system)
}

/// When passed the LabVIEW registry key this function will extract all installs it can find.
#[cfg(target_os = "windows")]
fn installations_from_labview_registry(labview_key: RegKey, bitness: Bitness, system: &mut SystemLabviewInstalls) -> Result<(), LabviewInstallError> {


    let filtered_keys = labview_key.enum_keys()
                                .filter_map(|name_result| { name_result.ok() })//Filter errors.
                                .filter(|name| name != "AddOns")
                                .filter(|name| name != "CurrentVersion");

    for version_key in filtered_keys {
        
        let item_key = labview_key.open_subkey(&version_key).map_err(|err| LabviewInstallError::RegKeyError(err, version_key.to_owned()))?;
        let key_contents = extract_install_details(item_key, bitness);

        if let Some(install) = key_contents {
            system.add_install(install);
        }
    }

    Ok(())

}

/// From the registry key, extract the install details.
/// Sometimes an install key will be empty. Returns none in this case.
#[cfg(target_os = "windows")]
fn extract_install_details(install_key: RegKey, bitness: Bitness) -> Option<LabviewInstall> {

    // Use version string as a test. If it exists try and get the others.
    // Think there must be a nicer way than the nested if/else.
    if let Ok(version_string) = install_key.get_value("VersionString") {

        if let Ok(install_path) = install_key.get_value::<String, &str>("Path") {
            let install = LabviewInstall {
                version: version_string,
                path: PathBuf::from(install_path),
                bitness
            };
        
            Some(install)
        }
        else {
            None
        }
    }
    else {
        None
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
