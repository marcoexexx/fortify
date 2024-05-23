#[derive(Debug)]
pub enum Error {
    CryptoError(crypto::symmetriccipher::SymmetricCipherError),
}
