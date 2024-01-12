use serde_derive::Deserialize;

use crate::rocket::common::config::mysql_database_config::MySqlDatabaseConfig;
use std::{fs, process};

use lazy_static::lazy_static;
use toml::de::Error;
use crate::rocket::common::config::auth_config::AuthConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub mysql_database_config: MySqlDatabaseConfig,
	pub auth_config: AuthConfig
}

pub fn init_app_config() -> Result<AppConfig, Error> { read_app_config() }

fn read_app_config() -> Result<AppConfig, Error> {
	let path = if cfg!(test) {
		"Config_test.toml"
	} else {
		"Config.toml"
	};
	let toml_str = fs::read_to_string(path).expect("Failed to read Cargo.toml file");
	toml::from_str(&toml_str)
}

lazy_static! {
	pub static ref APP_CONFIG: AppConfig = {
		match read_app_config() {
			Ok(common_config) => common_config,
			Err(err) => {
				println!("{}", err);
				process::exit(1)
			}
		}
	};
}