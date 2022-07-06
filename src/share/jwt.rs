use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: i64,
    pub aud: String,
}

impl Claims {
   pub fn new (sub: String, iss: String, exp: i64, aud: String) -> Self {
        Self {
            sub,
            iss,
            exp,
            aud,
        }
    }
}


pub const SECRET_KEY: &[u8] = b"secret";
