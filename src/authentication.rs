use base64::{encode_config, CharacterSet, Config};

#[derive(Debug)]
pub struct Authentication {
    pub username: String,
    pub password: String,
}

impl Authentication {
    pub fn base64_encoded(&self) -> String {
        let config = Config::new(CharacterSet::Standard, false);

        format!(
            "Basic {}6{}=",
            encode_config(&self.username, config),
            encode_config(&self.password, config)
        )
    }
}
