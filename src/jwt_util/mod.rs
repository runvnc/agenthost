extern crate jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, Header, get_current_timestamp, Validation, errors::{ErrorKind}};
use serde::{Deserialize, Serialize};
use hyper::http;
use axum::{
    http::{
        Request, Response, StatusCode
    }, 
};

use crate::{s};

const CODE: &str = "cvx$^G#%^nh3t9y83h$%^@#isfdhioeroisd";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub expires: usize,
}


pub fn create_token(user_id: &str) -> Result<String, (StatusCode, &'static str)> {
    let my_claims = Claims {
        username: user_id.to_owned(),
        expires: (get_current_timestamp() + 60*60*24*7) as usize,
    };
    let encoding_key = jwt::EncodingKey::from_secret(CODE.as_ref());
    let token = encode(&Header::default(), &my_claims, &encoding_key)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token creation failed."))?;
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims, (StatusCode, String)> {
    let decoding_key = jwt::DecodingKey::from_secret(CODE.as_ref());

    match decode::<Claims>(token, &decoding_key, &Validation::default()) {
        Ok(data) => Ok(data.claims),
        Err(err) => {
            // Detailed logging of the error
            let error_message = match *err.kind() {
                ErrorKind::InvalidToken => s!("Invalid token"),
                ErrorKind::InvalidSignature => s!("Invalid signature"),
                ErrorKind::InvalidEcdsaKey => s!("Invalid ECDSA key"),
                ErrorKind::InvalidRsaKey(ref msg) => s!(msg),
                ErrorKind::RsaFailedSigning => s!("RSA failed signing"),
                ErrorKind::InvalidAlgorithmName => s!("Invalid algorithm name"),
                ErrorKind::InvalidKeyFormat => s!("Invalid key format"),
                ErrorKind::MissingRequiredClaim(ref claim) => s!(claim),
                ErrorKind::ExpiredSignature => s!("Expired signature"),
                ErrorKind::InvalidIssuer => s!("Invalid issuer"),
                ErrorKind::InvalidAudience => s!("Invalid audience"),
                ErrorKind::InvalidSubject => s!("Invalid subject"),
                ErrorKind::ImmatureSignature => s!("Immature signature"),
                ErrorKind::InvalidAlgorithm => s!("Invalid algorithm"),
                ErrorKind::MissingAlgorithm => s!("Missing algorithm"),
                ErrorKind::Base64(ref err) => s!(err),
                ErrorKind::Json(ref err) => s!(err),
                ErrorKind::Utf8(ref err) => s!(err),
                ErrorKind::Crypto(ref err) => s!(err),
                _ => s!("Other JWT error"),
            };

            Err((StatusCode::UNAUTHORIZED, error_message))
        }
    }
}

/*
pub fn verify_token(token: &str) -> Result<Claims, (StatusCode, &'static str)> {
    let decoding_key = jwt::DecodingKey::from_secret(CODE.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token verification failed."))
        .map(|data| data.claims)
} */

