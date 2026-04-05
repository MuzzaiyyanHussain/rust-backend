use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub exp: usize,
}

pub fn create_token(email: &str) -> String {
    let claims = Claims {
        sub: email.to_string(),
        exp: 2000000000, 
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ).unwrap()
}

pub fn verify_token(token: &str) -> Option<Claims> {
    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    match result {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}