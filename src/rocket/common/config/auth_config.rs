use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
	pub secret_token: String,
	pub access_token: String
}
