use crate::utils;

pub struct ContentManager {
    pub password: String,
    pub key: [u8; 32],
}

impl ContentManager {
    pub fn new(password: &str) -> ContentManager {
        let key = utils::generate_random_32();

        ContentManager {
            password: String::from(password),
            key,
        }
    }
}
