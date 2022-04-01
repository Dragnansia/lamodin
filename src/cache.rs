use crate::error::Error;
use std::{fs, path::Path};

/// Clear application cache
pub fn clear(app_name: &str) -> Result<(), Error> {
    let directory_path = path(app_name).ok_or("Can't get cache path")?;
    let path = Path::new(&directory_path);

    if path.exists() {
        fs::remove_dir_all(path)?;
        fs::create_dir_all(&path)?;
    }

    Ok(())
}

/// Return cache path
pub fn path(app_name: &str) -> Option<String> {
    let path = format!(
        "{}/{}/purs/",
        dirs::cache_dir()?.as_path().to_str()?,
        app_name
    );

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path).ok()?;
    }

    Some(path)
}
