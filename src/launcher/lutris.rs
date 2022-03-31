use super::Launcher;
use crate::{error::Error, modifier::Modifier};
use std::{fs, path::Path};

pub struct Lutris {
    pub path: String,
    pub modifier_path: String,
    pub modifiers: Vec<Modifier>,
}

impl Lutris {
    pub fn new() -> Result<Self, Error> {
        let path = Self::path()?;
        let modifier_path = Self::modifier_path(&path)?;
        Ok(Self {
            path,
            modifiers: Self::all_modifiers(&modifier_path)?,
            modifier_path,
        })
    }

    fn path() -> Result<String, Error> {
        let data_dir = dirs::data_dir()
            .ok_or("Can't find home dir")?
            .to_str()
            .ok_or("err")?
            .to_string();

        let path = format!("{}{}", data_dir, "/lutris/");
        if Path::new(&path).exists() {
            Ok(path)
        } else {
            Err("Can't find any Lutris directory".into())
        }
    }

    fn modifier_path(path: &str) -> Result<String, Error> {
        let modifier_path = format!("{}runners/wine/", path);

        if Path::new(&modifier_path).exists() {
            return Ok(modifier_path);
        }

        fs::create_dir_all(&modifier_path)?;
        Ok(modifier_path)
    }

    fn all_modifiers(modifier_path: &String) -> Result<Vec<Modifier>, Error> {
        let mut modifiers = vec![];
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

            modifiers.push(Modifier { name, path });
        }

        modifiers.sort();
        modifiers.reverse();
        Ok(modifiers)
    }
}

impl Launcher for Lutris {
    fn containt_version(&self, name: &str) -> bool {
        self.modifiers.iter().any(|m| m.name.starts_with(name))
    }

    fn modifiers(&self) -> Vec<Modifier> {
        self.modifiers.clone()
    }
}
