use crate::error::Error;
use std::{fs, path::Path};

pub struct Steam {
    pub path: String,
    pub modifier_path: String,
}

impl Steam {
    pub fn new() -> Result<Self, Error> {
        let path = Steam::fpath()?;
        let modifier_path = Steam::ppath(&path)?;
        Ok(Self {
            path,
            modifier_path,
        })
    }

    // find steam path
    fn fpath() -> Result<String, Error> {
        let home_dir = dirs::home_dir()
            .ok_or("Can't find home dir")?
            .to_str()
            .ok_or("err")?
            .to_string();

        let steam_path = format!("{}{}", home_dir, "/.steam/");
        if Path::new(&steam_path).exists() {
            Ok(steam_path)
        } else {
            Err("Can't find any Steam directory".into())
        }
    }

    // Parse steam path to get proton path
    fn ppath(steam_path: &str) -> Result<String, Error> {
        let proton_path = format!("{}root/compatibilitytools.d/", steam_path);

        if Path::new(&proton_path).exists() {
            return Ok(proton_path);
        }

        fs::create_dir_all(&proton_path)?;
        Ok(proton_path)
    }
}
