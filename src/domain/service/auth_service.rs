use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
	domain::model::auth::claims::Claims
};
use crate::rocket::common::config::common_config::APP_CONFIG;

pub struct AuthService {
	claims: Claims
}

impl AuthService {
	pub fn new(claims: Claims) -> Self { Self { claims } }

	pub fn generate_jwt(&self) -> jsonwebtoken::errors::Result<String> {
		info!("Generating new JWT");
		encode(
			&Header::default(),
			&self.claims,
			&EncodingKey::from_secret(&APP_CONFIG.get().unwrap().auth_config.secret_token.as_ref())
		)
	}
}
