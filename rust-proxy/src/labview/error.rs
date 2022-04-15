use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LabVIEWError {
    #[error("Registration error talking to NI Service Locator - is it running?")]
    ServiceLocatorCommsError(#[source] ureq::Error),
    #[error("Bad Response from NI Service Location. Response code: {0}")]
    ServiceLocatorResponseError(u16),
    #[error("Process launch failed")]
    ProcessLaunchFailed(#[source] std::io::Error),
    #[error("VI to launch does not exist: \"{0}\"")]
    ViDoesNotExist(PathBuf),
    #[error("Nul characters in argument isn't allowed")]
    NullCharInArgument,
}
