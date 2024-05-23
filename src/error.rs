#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    CryptoError(crypto::symmetriccipher::SymmetricCipherError),
}
