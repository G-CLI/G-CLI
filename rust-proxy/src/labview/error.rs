use thiserror::Error;
#[derive(Error, Debug)]
pub enum LabVIEWError {
    #[error("Registration error talking to NI Service Locator - is it running?")]
    ServiceLocatorCommsError(#[source] ureq::Error),
    #[error("Bad Response from NI Service Location. Response code: {0}")]
    ServiceLocatorResponseError(u16)

}
