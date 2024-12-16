use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: i32,
    exp: usize,
}

pub struct Jwt {
    pub claims: Claims,
}

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration_time = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        user_id: id,
        exp: expiration_time as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = token.trim_start_matches("Bearer ").trim();

    match decode::<Claims>(
        &token,
        *DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(e) => Err(e.kind().to_owned()),
    }
}
