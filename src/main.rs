mod content;
mod encryptor;
mod error;
mod utils;

fn main() -> Result<(), error::Error> {
    let encryption = encryptor::Encryption::new();

    let password_manager = content::ContentManager::new("fish");

    let encrypted = encryption.encrypt(&password_manager)?;
    let decrypted = encryption.decrypt(&password_manager, &encrypted)?;

    assert_eq!(
        decrypted,
        password_manager.password.as_bytes(),
        "same bytes"
    );

    Ok(())
}
