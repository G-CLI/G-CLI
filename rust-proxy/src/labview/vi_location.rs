//! Functions to help with path management around VIs.
//!
//!

use std::ffi::{OsStr, OsString};
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
/// VILocation represents the location of a VI in disk.
///
/// While this can simply be a file it may also be contained within libraries
/// and in the future we may wish to provide some virtual expansion such as vi.lib.
///
/// VI location is constructed from the path provided by the user and provides functionality
/// to access these different levels and concerns.
pub struct VILocation {
    // The actual file on disk which contains the VI. Could still be relative etc.
    container: PathBuf,
    // Remains as the provided path
    vi: PathBuf,
}

impl VILocation {
    pub fn new(vi_path: &Path) -> Self {
        VILocation {
            container: get_file_container(vi_path),
            vi: vi_path.to_owned(),
        }
    }
    pub fn container(&self) -> &Path {
        &self.container
    }
    pub fn exists(&self) -> bool {
        self.container.exists()
    }
    pub fn labview_parameter(&self) -> OsString {
        self.vi.as_os_str().to_owned()
    }
    /// Returns the full canonical path to the VI inside it's container.
    /// This is required to match what LabVIEW returns from the `VI Path` constant.
    pub fn canonical_vi_path(&self) -> PathBuf {
        //Need to make this a full path here since that is what LabVIEW will use for it's ID.
        //Since we need to handle the container case canonicalize will error if it doesn't exist
        // Therefore we have to split and combine here.
        let mut full_path = canonicalize(&self.container).unwrap();
        let path_diff = self.vi.strip_prefix(&self.container).unwrap();
        full_path.push(path_diff);
        full_path
    }
}

impl std::fmt::Display for VILocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.vi.to_string_lossy().as_ref())
    }
}

/// Sometimes VIs can be valid but exist in container formats. e.g.
/// * .llb files
/// * packed libraries
///
/// This identifies if these containers exist in the path and return their path.
/// Otherwise it returns a clone of the VI path.
fn get_file_container(vi: &Path) -> PathBuf {
    for sub_path in vi.ancestors() {
        if let Some(extension) = sub_path.extension() {
            if extension == "lvlibp" || extension == "llb" {
                return sub_path.to_owned();
            }
        }
    }

    //if we reached this far, then no container.
    vi.to_owned()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_container_none() {
        let vi_path = PathBuf::from("/C/Test/test.vi");
        let location = VILocation::new(&vi_path);
        assert_eq!(vi_path, location.container());
    }

    #[test]
    fn test_get_container_packed_library() {
        let vi_path = PathBuf::from("/C/Test.lvlibp/test.vi");
        let location = VILocation::new(&vi_path);
        assert_eq!(PathBuf::from("/C/Test.lvlibp"), location.container());
    }

    #[test]
    fn test_get_container_packed_library_internal_dir() {
        let vi_path = PathBuf::from("/C/Test.lvlibp/folder/test.vi");
        let location = VILocation::new(&vi_path);
        assert_eq!(PathBuf::from("/C/Test.lvlibp"), location.container());
    }

    #[test]
    fn test_get_container_llb() {
        let vi_path = PathBuf::from("/C/Test.llb/test.vi");
        let location = VILocation::new(&vi_path);
        assert_eq!(PathBuf::from("/C/Test.llb"), location.container());
    }

    #[test]
    fn test_get_canonical_path_vi() {
        //This path must exist for this to work.
        let cargo_path = cargo_path();
        let mut canonical_path = PathBuf::from(cargo_path.clone());
        canonical_path.push("test_data/test.vi");
        let mut relative_path = PathBuf::from(cargo_path);
        relative_path.push("src/../test_data/test.vi");

        assert_eq!(
            VILocation::new(&relative_path).canonical_vi_path(),
            canonical_path
        );
    }

    #[test]
    fn test_get_canonical_path_packed_library() {
        //This path must exist for this to work.
        let cargo_path = cargo_path();
        let mut canonical_path = PathBuf::from(cargo_path.clone());
        canonical_path.push("test_data/test.lvlibp/folder/test.vi");
        let mut relative_path = PathBuf::from(cargo_path);
        relative_path.push("src/../test_data/test.lvlibp/folder/test.vi");

        assert_eq!(
            VILocation::new(&relative_path).canonical_vi_path(),
            canonical_path
        );
    }

    #[cfg(windows)]
    fn cargo_path() -> String {
        let env = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        //On windows canonicalise adds this prefix. We don't care for the IDs but need this to match in tests.
        String::from(r"\\?\") + &env
    }
    #[cfg(not(windows))]
    fn cargo_path() -> String {
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    }
}
