//! Install detection code for the windows platform.
//!
use std::path::PathBuf;

use winreg::enums::*;
use winreg::RegKey;

use super::installs::{Bitness, LabviewInstall, LabviewInstallError, SystemLabviewInstalls};

/// Scan the system for LabVIEW installs and return their details.
pub fn detect_installations() -> Result<SystemLabviewInstalls, LabviewInstallError> {
    let mut system = SystemLabviewInstalls::new();

    // The key structure here is only valid for 64 bit OS.
    // I think this is acceptible - I don't expect support for 32 bit OS to be a critical starting point.

    const BASE_KEY: &str = "SOFTWARE\\National Instruments\\LabVIEW";
    const BASE_KEY_WOW: &str = "SOFTWARE\\WOW6432Node\\National Instruments\\LabVIEW";

    let labview_32_key = try_open_key(BASE_KEY_WOW)?;
    let labview_64_key = try_open_key(BASE_KEY)?;

    if let Some(key) = labview_32_key {
        installations_from_labview_registry(key, Bitness::X86, &mut system)?;
    }

    if let Some(key) = labview_64_key {
        installations_from_labview_registry(key, Bitness::X64, &mut system)?;
    }

    Ok(system)
}

// Try opening the provided key, returning none if not found.
fn try_open_key(subkey: &str) -> Result<Option<RegKey>, LabviewInstallError> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(subkey)
        .map(Some)
        .or_else(|err| match err.kind() {
            std::io::ErrorKind::NotFound => Ok(None),
            _ => Err(LabviewInstallError::RegKeyError(err, subkey.to_owned())),
        })
}

/// When passed the LabVIEW registry key this function will extract all installs it can find.
fn installations_from_labview_registry(
    labview_key: RegKey,
    bitness: Bitness,
    system: &mut SystemLabviewInstalls,
) -> Result<(), LabviewInstallError> {
    let filtered_keys = labview_key
        .enum_keys()
        .filter_map(|name_result| name_result.ok()) //Filter errors.
        .filter(|name| name != "AddOns")
        .filter(|name| name != "CurrentVersion");

    for version_key in filtered_keys {
        let item_key = labview_key
            .open_subkey(&version_key)
            .map_err(|err| LabviewInstallError::RegKeyError(err, version_key.to_owned()))?;
        let key_contents = extract_install_details(item_key, bitness);

        if let Some(install) = key_contents {
            system.add_install(install);
        }
    }

    Ok(())
}

/// From the registry key, extract the install details.
/// Sometimes an install key will be empty. Returns none in this case.
fn extract_install_details(install_key: RegKey, bitness: Bitness) -> Option<LabviewInstall> {
    // Use version string as a test. If it exists try and get the others.
    // Think there must be a nicer way than the nested if/else.
    if let Ok(version_string) = install_key.get_value("VersionString") {
        if let Ok(install_path) = install_key.get_value::<String, &str>("Path") {
            let install = LabviewInstall {
                version: version_string,
                path: PathBuf::from(install_path),
                bitness,
            };

            Some(install)
        } else {
            None
        }
    } else {
        None
    }
}
