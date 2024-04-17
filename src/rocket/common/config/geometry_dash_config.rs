use lazy_static::lazy_static;
use serde_derive::Deserialize;

use crate::rocket::common::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize)]
pub struct GeometryDashConfig {
	pub gd_username: String,
	pub gd_password: String
}

lazy_static! {
	pub static ref GEOMETRY_DASH_CONFIG: &'static GeometryDashConfig =
		&APP_CONFIG.geometry_dash_config;
}
