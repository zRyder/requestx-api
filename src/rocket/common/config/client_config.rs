use lazy_static::lazy_static;
use serde_derive::Deserialize;
use crate::rocket::common::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub port: u16,
    pub discord_app_id: u64,
    pub discord_bot_admin_id: u64
}

lazy_static! {
	pub static ref CLIENT_CONFIG: &'static ClientConfig = &APP_CONFIG.client_config;
}