use config::{Config, ConfigError};
use serde_derive::Deserialize;

use crate::rocket::common::config::mysql_database_config::MySqlDatabaseConfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
	pub mysql_database_config: MySqlDatabaseConfig
}
