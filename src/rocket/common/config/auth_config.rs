use lazy_static::lazy_static;
use serde_derive::Deserialize;

use crate::rocket::common::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
	pub discord_app_id: String,
	pub secret_token: String,
	pub access_token: String,
	pub gd_username: String,
	pub gd_password: String
}

lazy_static! {
	pub static ref AUTH_CONFIG: &'static AuthConfig = &APP_CONFIG.auth_config;
}
