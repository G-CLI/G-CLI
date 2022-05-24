use super::installs::{Bitness, LabviewInstall, LabviewInstallError, SystemLabviewInstalls};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::fs;

const LABVIEW_INSTALL_DIR: &str = "/usr/local/natinst";

/// Scan the system for LabVIEW installs and return their details.
pub fn detect_installations() -> Result<SystemLabviewInstalls, LabviewInstallError> {
    let ni_paths = fs::read_dir(LABVIEW_INSTALL_DIR)
        .map_err(|e| LabviewInstallError::DirectoryError(e, LABVIEW_INSTALL_DIR.to_string()))?;

    let mut system = SystemLabviewInstalls::new();

    for path in ni_paths {
        let entry = path
            .map_err(|e| LabviewInstallError::DirectoryError(e, LABVIEW_INSTALL_DIR.to_string()))?;
        if entry.file_type().expect("Cant get file type").is_dir() {
            let labview_details =
                parse_labview_from_folder_name(&entry.file_name().to_string_lossy());

            if let Some((version, bitness)) = labview_details {
                let install = LabviewInstall {
                    path: entry.path(),
                    version,
                    bitness,
                };
                system.add_install(install);
            }
        }
    }

    return Ok(system);
}

/// Takes a folder name and either returns a tuple of LabVIEW version and bitness
/// or none if the pattern doesn't match.
fn parse_labview_from_folder_name(name: &str) -> Option<(String, Bitness)> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"labview-([^-]*)-(\d{2})")
            .case_insensitive(true)
            .build()
            .unwrap();
    }
    let captures = RE.captures(name);

    if let Some(capture_items) = captures {
        let version = capture_items[1].to_string();
        let bitness_string = &capture_items[2];
        match bitness_string {
            "64" => Some((version, Bitness::X64)),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs::ReadDir;

    use super::*;

    #[test]
    fn can_parse_valid_64bit_labview_dir() {
        let result = parse_labview_from_folder_name("LabVIEW-2019-64");

        assert_eq!(result, Some(("2019".to_owned(), Bitness::X64)));
    }

    #[test]
    fn can_parse_valid_64bit_labview_dir_case() {
        let result = parse_labview_from_folder_name("labview-2019-64");

        assert_eq!(result, Some(("2019".to_owned(), Bitness::X64)));
    }

    #[test]
    fn invalid_labview_folder() {
        let result = parse_labview_from_folder_name("lvmerge");

        assert_eq!(result, None);
    }

    #[test]
    fn invalid_labview_folder_starts_right() {
        let result = parse_labview_from_folder_name("LabVIEW");

        assert_eq!(result, None);
    }

    #[test]
    fn invalid_labview_folder_starts_right_possible_32bit() {
        let result = parse_labview_from_folder_name("LabVIEW-2019");

        assert_eq!(result, None);
    }
}
