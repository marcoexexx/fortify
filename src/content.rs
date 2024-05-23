use crate::utils;

pub struct ContentManager {
    pub ctx: String,
    pub key: [u8; 32],
}

impl ContentManager {
    pub fn new(ctx: &str) -> ContentManager {
        let key = utils::generate_random_32();

        ContentManager {
            ctx: String::from(ctx),
            key,
        }
    }
}
