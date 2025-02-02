use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub discord_app_id: u64,
    pub discord_bot_admin_id: u64,
    pub request_config_path: String,
}
