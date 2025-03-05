use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uuid: String,
    exp: usize,
}

pub fn generate_jwt_token(uuid: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        uuid: uuid.to_owned(),
        exp: 10000000000,
    };
    let key = b"secret"; // Define your secret key here
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    )?;
    Ok(token)
}
