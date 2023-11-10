extern crate jsonwebtoken as jwt;
use jwt::{encode, decode, Header, Algorithm, Validation};
use serde::{Serialize, Deserialize};
use warp::Rejection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub expires: usize,
}

pub fn create_token(user_id: &str) -> Result<String, Rejection> {
    let my_claims = Claims {
        username: user_id.to_owned(),
        expires: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let key = "secret";
    let encoding_key = jwt::EncodingKey::from_secret(key.as_ref());
    encode(&Header::default(), &my_claims, &encoding_key)
        .map_err(|_| warp::reject::custom(SimpleRejection("Token creation failed".into())))
}

pub fn verify_token(token: &str) -> Result<Claims, Rejection> {
    let key = "secret";
    let decoding_key = jwt::DecodingKey::from_secret(key.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map_err(|_| warp::reject::custom(SimpleRejection("Token verification failed".into())))
        .map(|data| data.claims)
}

// Import SimpleRejection from the errors module.
use crate::errors::SimpleRejection;
