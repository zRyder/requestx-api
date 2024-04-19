use std::sync::Mutex;

use chrono::Duration;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

use crate::rocket::common::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
	pub host: String,
	pub port: u16,
	pub discord_app_id: u64,
	pub discord_bot_admin_id: u64,
	pub cooldown_duration: u16,
	pub enable_requests: bool,
	pub enable_gd_requests: bool
}

lazy_static! {
	pub static ref CLIENT_CONFIG: &'static ClientConfig = &APP_CONFIG.client_config;
}

lazy_static! {
	pub static ref COOLDOWN_DURATION: Mutex<Duration> =
		Mutex::new(Duration::minutes(CLIENT_CONFIG.cooldown_duration as i64));
}

lazy_static! {
	pub static ref ENABLE_REQUESTS: Mutex<bool> = Mutex::new(CLIENT_CONFIG.enable_requests);
}

lazy_static! {
	pub static ref ENABLE_GD_REQUESTS: Mutex<bool> = Mutex::new(CLIENT_CONFIG.enable_gd_requests);
}
