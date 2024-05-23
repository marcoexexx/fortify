use crate::content::ContentManager;
use crate::error;
use crypto::aes::KeySize;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};

pub struct Encryption;

impl Encryption {
    pub fn new() -> Encryption {
        Encryption
    }

    pub fn get_encryptor(key: &[u8]) -> Box<dyn Encryptor> {
        let encryptor = crypto::aes::cbc_encryptor(
            KeySize::KeySize256,
            key,
            &[0u8; 16],
            crypto::blockmodes::PkcsPadding,
        );

        encryptor
    }

    pub fn get_decryptor(key: &[u8]) -> Box<dyn Decryptor> {
        let decryptor = crypto::aes::cbc_decryptor(
            KeySize::KeySize256,
            key,
            &[0u8; 16],
            crypto::blockmodes::PkcsPadding,
        );

        decryptor
    }

    pub fn encrypt(&self, ctx: &ContentManager) -> Result<Vec<u8>, error::Error> {
        let mut encryptor = Self::get_encryptor(&ctx.key);

        let mut final_result = Vec::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(ctx.ctx.as_bytes());
        let mut buffer = [0u8; 4096];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor
                .encrypt(&mut read_buffer, &mut write_buffer, true)
                .map_err(error::Error::CryptoError)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter());

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(final_result)
    }

    pub fn decrypt(&self, ctx: &ContentManager, data: &[u8]) -> Result<Vec<u8>, error::Error> {
        let mut decryptor = Self::get_decryptor(&ctx.key);

        let mut final_result = Vec::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
        let mut buffer = [0u8; 4096];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor
                .decrypt(&mut read_buffer, &mut write_buffer, true)
                .map_err(error::Error::CryptoError)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter());

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(final_result)
    }
}
