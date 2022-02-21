use crate::error::Error;
use std::fs::File;
use tar::Archive;

/// Install archive on the new path
pub fn install(file_path: &str, install_path: &str) -> Result<(), Error> {
    let archive = File::open(file_path)?;
    let mut archive = Archive::new(&archive);
    archive.unpack(install_path)?;

    Ok(())
}
