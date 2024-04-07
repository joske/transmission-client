use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};

#[derive(Debug)]
pub struct Authentication {
    pub username: String,
    pub password: String,
}

impl Authentication {
    pub fn base64_encoded(&self) -> String {
        let auth = format!("{}:{}", &self.username, &self.password);
        format!("Basic {}=", STANDARD_NO_PAD.encode(auth))
    }
}
