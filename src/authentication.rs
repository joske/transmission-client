use base64::{encode_config, CharacterSet, Config};

#[derive(Debug)]
pub struct Authentication {
    pub username: String,
    pub password: String,
}

impl Authentication {
    pub fn base64_encoded(&self) -> String {
        let config = Config::new(CharacterSet::Standard, false);
        let auth = format!("{}:{}", &self.username, &self.password);

        format!("Basic {}=", encode_config(auth, config))
    }
}
