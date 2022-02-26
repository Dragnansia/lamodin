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
        let home_dir = dirs::data_dir()
            .ok_or("Can't find home dir")?
            .to_str()
            .ok_or("err")?
            .to_string();

        let steam_path = format!("{}{}", home_dir, "/lutris/");
        if Path::new(&steam_path).exists() {
            Ok(steam_path)
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
        let mut array: Vec<String> = Vec::new();
        for pe in fs::read_dir(modifier_path)? {
            let pe = pe?;
            array.push(
                pe.path()
                    .file_name()
                    .ok_or("file name")?
                    .to_str()
                    .ok_or("to str")?
                    .to_string(),
            );
        }

        array.sort();
        array.reverse();
        Ok(array)
    }

    pub fn is_installed(&self, version: &String) -> bool {
        self.modifiers.contains(version)
    }
}
