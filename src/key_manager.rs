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

    pub fn save_key(&self) -> Result<(), error::Error> {
        let mut buf = File::create(&self.name).map_err(error::Error::IoError)?;

        buf.write_all(&self.key).map_err(error::Error::IoError)
    }
}

impl From<&path::Path> for KeyManager {
    fn from(path: &path::Path) -> KeyManager {
        let mut buf = File::open(path).expect("Failed to read key file");
        let mut key = [0u8; 32];

        buf.read_exact(&mut key).expect("Failed to store key");

        KeyManager {
            name: String::from(path.file_name().unwrap().to_str().unwrap()),
            key,
        }
    }
}
