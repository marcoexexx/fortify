mod content;
mod encryptor;
mod error;
mod utils;

fn main() -> Result<(), error::Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_content() {
        let encryption = encryptor::Encryption::new();

        let content = content::ContentManager::new("fish");

        let encrypted = encryption
            .encrypt(&content)
            .expect("Unable to encrypt content");
        let decrypted = encryption
            .decrypt(&content, &encrypted)
            .expect("Unable to decrypt to content");

        assert_eq!(content.ctx.as_bytes(), decrypted);
    }
}
