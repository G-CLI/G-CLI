use super::installs::{LabviewInstallError, SystemLabviewInstalls};

/// Scan the system for LabVIEW installs and return their details.
pub fn detect_installations() -> Result<SystemLabviewInstalls, LabviewInstallError> {
    Ok(SystemLabviewInstalls::new())
}
