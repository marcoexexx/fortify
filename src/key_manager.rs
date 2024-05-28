use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path;

use crate::{error, utils};

pub struct KeyManager {
    pub name: String,
    pub key: [u8; 32],
}

impl KeyManager {
    pub fn new(name: &str) -> KeyManager {
        let key = utils::generate_random_32();

        KeyManager {
            name: String::from(name),
            key,
        }
    }

    pub fn save_key(&mut self) -> Result<(), error::Error> {
        let path = path::Path::new(&self.name);
        println!("{:?}", path);
        let mut buf = File::create(&path)?;

        self.name = path
            .file_name()
            .unwrap_or(OsStr::new("new-key"))
            .to_str()
            .unwrap_or("new-key")
            .to_string();

        buf.write_all(&self.key)?;

        Ok(())
    }
}

impl From<&path::PathBuf> for KeyManager {
    fn from(path: &path::PathBuf) -> KeyManager {
        let mut buf = File::open(path).expect("Failed to read key file");
        let mut key = [0u8; 32];

        buf.read_exact(&mut key).expect("Failed to store key");

        KeyManager {
            name: String::from(path.file_name().unwrap().to_str().unwrap()),
            key,
        }
    }
}
