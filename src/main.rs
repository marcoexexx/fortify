use std::fs::File;
use std::io::{Read, Write};
use std::path;

use key_manager::KeyManager;

mod encryptor;
mod error;
mod key_manager;
mod utils;

const HIDE_DIR: &str = "";
const KEY_PATH: &str = "";
const IGNORE_LIST: [&str; 5] = ["node_modules", "target", ".git", "dist", "build"];

fn encrypt_file_contents(
    path: &path::Path,
    key_manager: &KeyManager,
    ignore_list: &[&str],
) -> Result<(), error::Error> {
    if path.is_dir() {
        if ignore_list.contains(&path.file_name().unwrap().to_str().unwrap()) {
            println!("Skip: {}", path.display());

            return Ok(());
        }

        let paths = path.read_dir().map_err(error::Error::IoError)?;

        for path in paths {
            if let Ok(entry) = path {
                let sub_path = entry.path();
                encrypt_file_contents(&sub_path, key_manager, ignore_list)?;
            }
        }

        return Ok(());
    }

    let mut fp = File::open(path).map_err(error::Error::IoError)?;
    let mut buf = Vec::new();

    fp.read_to_end(&mut buf).map_err(error::Error::IoError)?;

    let encryption = encryptor::Encryption::new();
    let encrypted = encryption.encrypt(&key_manager.key, &buf)?;

    let mut fp = File::options()
        .write(true)
        .open(path)
        .map_err(error::Error::IoError)?;

    fp.write_all(&encrypted).map_err(error::Error::IoError)?;

    println!("[ SUCCESS ]: success encryption");

    Ok(())
}

fn decrypt_file_contents(
    path: &path::Path,
    key_manager: &KeyManager,
    ignore_list: &[&str],
) -> Result<(), error::Error> {
    if path.is_dir() {
        if ignore_list.contains(&path.file_name().unwrap().to_str().unwrap()) {
            println!("Skip: {}", path.display());

            return Ok(());
        }

        let paths = path.read_dir().map_err(error::Error::IoError)?;

        for path in paths {
            if let Ok(entry) = path {
                let sub_path = entry.path();
                decrypt_file_contents(&sub_path, key_manager, ignore_list)?;
            }
        }

        return Ok(());
    }

    let mut fp = File::open(path).map_err(error::Error::IoError)?;
    let mut buf = Vec::new();

    fp.read_to_end(&mut buf).map_err(error::Error::IoError)?;

    let encryption = encryptor::Encryption::new();
    let decrypted = encryption.decrypt(&key_manager.key, &buf)?;

    let mut fp = File::options()
        .write(true)
        .append(false)
        .truncate(true)
        .open(path)
        .map_err(error::Error::IoError)?;

    fp.write_all(&decrypted).map_err(error::Error::IoError)?;

    println!("[ SUCCESS ]: success decryption");

    Ok(())
}

fn main() -> Result<(), error::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let is_retrieve_mode = args.get(1).map_or(false, |x| x == "--retrieve");

    let path = path::Path::new(HIDE_DIR);

    let key_manager =
        key_manager::KeyManager::from(&path::PathBuf::from(&format!("{KEY_PATH}/.fish")));

    if !is_retrieve_mode {
        encrypt_file_contents(&path, &key_manager, &IGNORE_LIST)?;
    } else {
        decrypt_file_contents(&path, &key_manager, &IGNORE_LIST)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_content() {
        let text = String::from("hello world");
        let encryption = encryptor::Encryption::new();

        let mut key_manager = key_manager::KeyManager::new(&format!("{KEY_PATH}/.fish"));
        key_manager.save_key().expect("Unable to save kay");

        let encrypted = encryption
            .encrypt(&key_manager.key, text.as_bytes())
            .expect("Unable to encrypt content");
        let decrypted = encryption
            .decrypt(&key_manager.key, &encrypted)
            .expect("Unable to decrypt to content");

        assert_eq!(text.as_bytes(), decrypted);
    }
}
