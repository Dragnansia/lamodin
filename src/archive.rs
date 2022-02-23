use crate::error::Error;
use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

/// Install archive on the new path
///
/// # Exemple
///
/// ```
/// use purs::{archive::install, error::Error};
///
/// fn func() -> Result<(), Error> {
///     install("./archive.tar.gz", "new_dir/for_tar")?;
///     Ok(())
/// }
/// ```
pub fn install(file_path: &str, install_path: &str) -> Result<(), Error> {
    let archive = File::open(file_path)?;
    let decoder = GzDecoder::new(archive);
    let mut archive = Archive::new(decoder);
    archive.unpack(install_path)?;

    Ok(())
}
