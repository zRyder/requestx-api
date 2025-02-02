use std::{
	collections::HashMap,
	env,
	env::home_dir,
	fs,
	path::{Path, PathBuf},
	str::FromStr,
	sync::OnceLock
};

use config::{Config, ConfigError, File, FileFormat};
use serde_derive::Deserialize;
use tokio::{fs::OpenOptions, io::AsyncReadExt, sync::RwLock};

use crate::rocket::common::config::{
	auth_config::AuthConfig,
	geometry_dash_config::GeometryDashConfig,
	mysql_database_config::MySqlDatabaseConfig,
	request_config::{RequestConfig, REQUEST_CONFIG},
	server_config::ServerConfig
};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub server_config: ServerConfig,
	pub mysql_database_config: MySqlDatabaseConfig,
	pub auth_config: AuthConfig,
	pub geometry_dash_config: GeometryDashConfig
}

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub async fn init_app_config() -> Result<(), ConfigError> {
	let mut app_config = match read_app_config() {
		Ok(app_config) => app_config,
		Err(read_config_error) => return Err(read_config_error)
	};

	init_request_config(&mut app_config).await;
	APP_CONFIG
		.set(app_config)
		.expect("Unable to initialize app config");
	Ok(())
}

fn read_app_config() -> Result<AppConfig, ConfigError> {
	let env_vars: HashMap<String, String> = env::vars().collect();
	let mut settings = Config::builder();

	let handlebars = handlebars::Handlebars::new();
	let template_string;

	if cfg!(test) {
		template_string =
			fs::read_to_string("Config_test.toml").expect("Unable to open test configuration file");
	} else {
		template_string =
			fs::read_to_string("Config.toml").expect("Unable to open configuration file");
	};

	let rendered = handlebars
		.render_template(&template_string, &env_vars)
		.expect("Unable to render template");
	settings = settings.add_source(File::from_str(rendered.as_str(), FileFormat::Toml));
	settings.build().unwrap().try_deserialize::<AppConfig>()
}

async fn init_request_config(app_config: &mut AppConfig) {
	match OpenOptions::new()
		.read(true)
		.open(&app_config.server_config.request_config_path)
		.await
	{
		Ok(mut request_config_file) => {
			let mut request_config_buffer = String::new();
			if let Err(request_config_read_error) = request_config_file
				.read_to_string(&mut request_config_buffer)
				.await
			{
				error!(
					"unable to read request config file: {}",
					request_config_read_error
				)
			}
			if let Ok(request_config_from_file) =
				toml::from_str::<RequestConfig>(&request_config_buffer)
			{
				REQUEST_CONFIG
					.set(RwLock::new(request_config_from_file))
					.expect("unable to set request configuration");
			} else {
				warn!("Unable to initialize request config, setting to default value");
				REQUEST_CONFIG
					.set(RwLock::new(RequestConfig::default()))
					.expect("unable to set default request configuration");
			}
		}
		Err(client_config_error) => {
			warn!(
				"unable to open request config file, setting to default value: {}",
				client_config_error
			);
			REQUEST_CONFIG
				.set(RwLock::new(RequestConfig::default()))
				.expect("unable to set default request configuration");
		}
	};
}
