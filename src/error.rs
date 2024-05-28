#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    CryptoError(crypto::symmetriccipher::SymmetricCipherError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<crypto::symmetriccipher::SymmetricCipherError> for Error {
    fn from(value: crypto::symmetriccipher::SymmetricCipherError) -> Self {
        Self::CryptoError(value)
    }
}
