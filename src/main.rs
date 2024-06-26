use std::fs::File;
use std::io::{Read, Write};
use std::path;

use key_manager::KeyManager;

mod encryptor;
mod error;
mod key_manager;
mod utils;

const KEY_FILE: &str = "/home/marco/@personal/.me/fortify/.fish";
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

        let paths = path.read_dir()?;

        for path in paths {
            if let Ok(entry) = path {
                let sub_path = entry.path();
                encrypt_file_contents(&sub_path, key_manager, ignore_list)?;
            }
        }

        return Ok(());
    }

    let mut fp = File::open(path)?;
    let mut buf = Vec::new();

    fp.read_to_end(&mut buf)?;

    let encryption = encryptor::Encryption::new();
    let encrypted = encryption.encrypt(&key_manager.key, &buf)?;

    let mut fp = File::options()
        .write(true)
        .open(path)?;

    fp.write_all(&encrypted)?;

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

        let paths = path.read_dir()?;

        for path in paths {
            if let Ok(entry) = path {
                let sub_path = entry.path();
                decrypt_file_contents(&sub_path, key_manager, ignore_list)?;
            }
        }

        return Ok(());
    }

    let mut fp = File::open(path)?;
    let mut buf = Vec::new();

    fp.read_to_end(&mut buf)?;

    let encryption = encryptor::Encryption::new();
    let decrypted = encryption.decrypt(&key_manager.key, &buf)?;

    let mut fp = File::options()
        .write(true)
        .append(false)
        .truncate(true)
        .open(path)?;

    fp.write_all(&decrypted)?;

    println!("[ SUCCESS ]: success decryption");

    Ok(())
}

fn main() -> Result<(), error::Error> {
    let args = std::env::args().collect::<Vec<String>>();

    let hide_dir = args.get(1).expect("Unable to get hide path");
    let is_retrieve_mode = args.get(2).map_or(false, |x| x == "--retrieve");

    let path = path::Path::new(hide_dir);

    let key_manager = key_manager::KeyManager::from(&path::PathBuf::from(KEY_FILE));

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

        let mut key_manager = key_manager::KeyManager::new(KEY_FILE);
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
