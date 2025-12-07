use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeometryDashConfig {
	pub gd_username: String,
	pub gd_password: String,
}
