extern crate jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use hyper::http;
use axum::{
    http::{
        Request, Response, StatusCode
    }, 
};

const CODE: &str = "cvx$^G#%^nh3t9y83h$%^@#isfdhioeroisd";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub expires: usize,
}


pub fn create_token(user_id: &str) -> Result<String, (StatusCode, &'static str)> {
    let my_claims = Claims {
        username: user_id.to_owned(),
        expires: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let encoding_key = jwt::EncodingKey::from_secret(CODE.as_ref());
    let token = encode(&Header::default(), &my_claims, &encoding_key)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token creation failed."))?;
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims, (StatusCode, &'static str)> {
    let decoding_key = jwt::DecodingKey::from_secret(CODE.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token verification failed."))
        .map(|data| data.claims)
}
