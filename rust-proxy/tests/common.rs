use std::ffi::OsStr;

const INTEGRATION_VI_FOLDER: &str = "../LabVIEW Source/integration-tests";
const INTEGRATION_EXE_FOLDER: &str = "../Builds";

pub fn integration_vi_path(vi: &str) -> String {
    format!("{}/{}", INTEGRATION_VI_FOLDER, vi)
}

pub fn integration_build_path(vi: &str) -> String {
    format!("{}/{}", INTEGRATION_EXE_FOLDER, vi)
}

pub fn g_cli_args<'a>(
    vi: &'a str,
    vi_args: impl IntoIterator<Item = &'a str>,
) -> impl IntoIterator<Item = &'a OsStr> {
    [vi, "--"]
        .into_iter()
        .chain(vi_args.into_iter())
        .map(OsStr::new)
}
