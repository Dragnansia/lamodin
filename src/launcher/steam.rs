use super::Launcher;
use crate::{error::Error, modifier::Modifier};
use std::{fs, path::Path};

/// Steam infos
pub struct Steam {
    /// Basic path
    pub path: String,

    /// Modifier path of steam
    pub modifier_path: String,

    /// List of all `Modifier` found on modifier path
    pub modifiers: Vec<Modifier>,
}

impl Steam {
    pub fn new() -> Result<Self, Error> {
        let path = Steam::fpath()?;
        let modifier_path = Steam::ppath(&path)?;
        Ok(Self {
            path,
            modifiers: Self::all_modifiers(&modifier_path)?,
            modifier_path,
        })
    }

    /// find steam path
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

    /// Parse steam path to get proton path
    fn ppath(steam_path: &str) -> Result<String, Error> {
        let proton_path = format!("{}root/compatibilitytools.d/", steam_path);

        if Path::new(&proton_path).exists() {
            return Ok(proton_path);
        }

        fs::create_dir_all(&proton_path)?;
        Ok(proton_path)
    }

    /// Find all modifiers
    fn all_modifiers(modifier_path: &String) -> Result<Vec<Modifier>, Error> {
        let mut array = vec![];
        for pe in fs::read_dir(modifier_path)? {
            let pe = pe?;
            let name = pe
                .path()
                .file_name()
                .ok_or("file name")?
                .to_str()
                .ok_or("to str")?
                .to_string();

            let path = pe.path().to_str().ok_or("path name")?.to_string();

            array.push(Modifier { name, path });
        }

        array.sort();
        array.reverse();
        Ok(array)
    }
}

impl Launcher for Steam {
    fn containt_version(&self, name: &str) -> Option<Modifier> {
        self.modifiers
            .iter()
            .find(|m| m.name.contains(name))
            .cloned()
    }

    fn modifiers(&self) -> Vec<Modifier> {
        self.modifiers.clone()
    }
}
