use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct JwtValidator {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtValidator {
    pub fn new(secret: &str) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        }
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        decode::<JwtClaims>(token, &self.decoding_key, &self.validation)
            .map(|data| data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}