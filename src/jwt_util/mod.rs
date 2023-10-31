extern crate jsonwebtoken as jwt;
use jwt::{encode, decode, Header, Algorithm, Validation};
use serde::{Serialize, Deserialize};
use warp::Rejection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(user_id: &str) -> Result<String, Rejection> {
    let my_claims = Claims {
        sub: user_id.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let key = "secret";
    encode(&Header::default(), &my_claims, key.as_ref())
        .map_err(|_| warp::reject::custom("Token creation failed"))
}

pub fn verify_token(token: &str) -> Result<Claims, Rejection> {
    let key = "secret";
    decode::<Claims>(token, key.as_ref(), &Validation::default())
        .map_err(|_| warp::reject::custom("Token verification failed"))
        .map(|data| data.claims)
}

