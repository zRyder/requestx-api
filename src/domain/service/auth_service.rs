use jsonwebtoken::{encode, EncodingKey, Header};
use crate::domain::model::auth::claims::Claims;
use crate::rocket::common::config::auth_config::AUTH_CONFIG;

pub struct AuthService {
    claims: Claims
}

impl AuthService {
    pub fn new(claims: Claims) -> Self {
        Self {
            claims,
        }
    }

    pub fn generate_jwt(&self) -> jsonwebtoken::errors::Result<String> {
        encode(
            &Header::default(),
            &self.claims,
            &EncodingKey::from_secret(&AUTH_CONFIG.secret_token.as_ref())
        )
    }
}