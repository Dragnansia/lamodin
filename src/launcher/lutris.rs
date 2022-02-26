use crate::error::Error;
use std::{fs, path::Path};

pub struct Lutris {
    pub path: String,
    pub modifier_path: String,
    pub modifiers: Vec<String>,
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

    fn all_modifiers(modifier_path: &String) -> Result<Vec<String>, Error> {
        let mut modifiers: Vec<String> = Vec::new();
        for pe in fs::read_dir(modifier_path)? {
            let pe = pe?;
            modifiers.push(
                pe.path()
                    .file_name()
                    .ok_or("file name")?
                    .to_str()
                    .ok_or("to str")?
                    .to_string(),
            );
        }

        modifiers.sort();
        modifiers.reverse();
        Ok(modifiers)
    }

    pub fn is_installed(&self, version: &String) -> bool {
        self.modifiers.contains(version)
    }
}
