use super::error::LabVIEWError;
use super::installs::LabviewInstall;
use super::vi_location::VILocation;
use std::path::Path;
use ureq::get;

pub struct Registration {
    id: String,
}

impl Registration {
    pub fn register(
        vi: &VILocation,
        install: &LabviewInstall,
        port: &u16,
    ) -> Result<Registration, LabVIEWError> {
        let id = generate_registration_id(&vi.canonical_vi_path(), install);
        // The response we want the discovery service to give. I'm not sure if these need further escaping but so far it works
        let base_response = "HTTP/1.0 200 OK%0D%0AServer: Service Locator%0D%0APragma: no-cache%0D%0AConnection: Close%0D%0AContent-Length: 12%0D%0AContent-Type: text/html%0D%0A%0D%0A";
        let url = format!(
            "http://localhost:3580/publish?{}={}Port={}%0D%0A",
            id, base_response, port
        );

        let response = get(&url)
            .call()
            .map_err(|e| LabVIEWError::ServiceLocatorCommsError(e))?;

        let status_code = response.status();

        if status_code > 299 {
            Err(LabVIEWError::ServiceLocatorResponseError(status_code))
        } else {
            Ok(Registration { id })
        }
    }

    /// Unregisters the port with the service locator and consumes the registration object.
    pub fn unregister(self) -> Result<(), LabVIEWError> {
        let response = get(&format!("http://localhost:3580/delete?{}", self.id))
            .call()
            .map_err(|e| LabVIEWError::ServiceLocatorCommsError(e))?;

        let status_code = response.status();

        if status_code > 299 {
            Err(LabVIEWError::ServiceLocatorResponseError(status_code))
        } else {
            Ok(())
        }
    }
}

/// Generates an ID unique to the install and VI path.
/// Path should be the full path to the VI.
fn generate_registration_id(vi_path: &Path, install: &LabviewInstall) -> String {
    let path_string = vi_path.to_string_lossy();
    // The extra [..] is required on the pattern array to get the format correct.
    let reg_name = path_string.replace(&[':', '\\', '.', ' ', '/', '?'][..], "");

    format!(
        "cli/{}/{}/{}",
        install.major_version(),
        install.bitness,
        reg_name
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::labview::installs::Bitness;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_builds_the_correct_registration_id_32bit() {
        let install = LabviewInstall {
            path: PathBuf::from("C:\\LabVIEW.exe"),
            version: String::from("2011 SP1"),
            bitness: Bitness::X86,
        };

        let result = generate_registration_id(Path::new("C:\\myVI.vi"), &install);

        assert_eq!(String::from("cli/2011/32bit/CmyVIvi"), result);
    }

    #[test]
    fn test_builds_the_correct_registration_id_64bit() {
        let install = LabviewInstall {
            path: PathBuf::from("C:\\LabVIEW.exe"),
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
        };

        let result = generate_registration_id(Path::new("C:\\myVI.vi"), &install);

        assert_eq!(String::from("cli/2011/64bit/CmyVIvi"), result);
    }

    #[test]
    fn test_builds_the_correct_registration_id_forward_slash_64bit() {
        let install = LabviewInstall {
            path: PathBuf::from("C:\\LabVIEW.exe"),
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
        };

        let result = generate_registration_id(Path::new("/C/myVI.vi"), &install);

        assert_eq!(String::from("cli/2011/64bit/CmyVIvi"), result);
    }

    #[test]
    /// Question marks can appear with UNC paths on windows \\?\C:\test.vi for example.
    /// This ensures they arent formated.
    fn test_builds_the_correct_registration_id_removes_question_marks() {
        let install = LabviewInstall {
            path: PathBuf::from("C:\\LabVIEW.exe"),
            version: String::from("2011 SP1"),
            bitness: Bitness::X64,
        };

        let result = generate_registration_id(Path::new("\\\\?\\C:\\myVI.vi"), &install);

        assert_eq!(String::from("cli/2011/64bit/CmyVIvi"), result);
    }
}
