use std::fs::File;
use std::io::{Read, Write};
use std::path::{self, Path};

use key_manager::KeyManager;

mod encryptor;
mod error;
mod key_manager;
mod utils;

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
    let mut buf = String::new();

    fp.read_to_string(&mut buf).map_err(error::Error::IoError)?;

    let encryption = encryptor::Encryption::new();
    let encrypted = encryption.encrypt(&key_manager.key, buf.as_bytes())?;

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
    let path = path::Path::new("_test_dir");
    let ignore_list = ["node_modules", "target"];

    let key_manager = key_manager::KeyManager::from(Path::new("fish"));
    let do_hide = true;

    if do_hide {
        encrypt_file_contents(&path, &key_manager, &ignore_list)?;
    } else {
        decrypt_file_contents(&path, &key_manager, &ignore_list)?;
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

        let key_manager = key_manager::KeyManager::new("fish");
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
