use crate::db::schemas::Role;
use crate::error::Error;
use crate::utils::Result;
use chrono::prelude::*;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        JwtService { secret }
    }
    pub fn create_jwt(&self, uid: &str, role: &Role) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: uid.to_owned(),
            role: role.to_string(),
            exp: expiration as usize,
        };
        let header = Header::new(Algorithm::HS512);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| Error::JWTTokenCreationError)
    }

    pub fn validate(&self, jwt: String) -> Result<TokenData<Claims>> {
        decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .map_err(|_| Error::JWTTokenError)
    }
}
