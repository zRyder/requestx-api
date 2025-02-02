use chrono::Duration;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::rocket::common::config::{common_config::APP_CONFIG, request_config::REQUEST_CONFIG};

pub struct RequestManagerService {}

impl RequestManagerService {
	pub async fn set_request_cooldown(&self, duration_in_minutes: u64) {
		let guard = &mut REQUEST_CONFIG.get().unwrap().write().await;
		guard.cooldown_duration = duration_in_minutes;
		info!("Request cooldown set to {}", duration_in_minutes)
	}

	pub async fn get_request_cooldown(&self) -> Duration {
		let guard = &mut REQUEST_CONFIG.get().unwrap().read().await;
		Duration::minutes(guard.cooldown_duration as i64)
	}

	pub async fn set_enable_request(&self, enable_requests: bool) {
		let guard = &mut REQUEST_CONFIG.get().unwrap().write().await;
		guard.enable_requests = enable_requests;
		info!("Enable requests toggled to {}", enable_requests)
	}

	pub async fn get_enable_request(&self) -> bool {
		let guard = &mut REQUEST_CONFIG.get().unwrap().read().await;
		guard.enable_requests
	}

	pub async fn set_enable_gd_request(&self, enable_gd_requests: bool) {
		let guard = &mut REQUEST_CONFIG.get().unwrap().write().await;
		guard.enable_gd_requests = enable_gd_requests;
		info!("Enable GD requests toggled to {}", enable_gd_requests)
	}

	pub async fn get_enable_gd_request(&self) -> bool {
		let guard = &mut REQUEST_CONFIG.get().unwrap().read().await;
		guard.enable_gd_requests
	}

	pub async fn update_client_config_file(&self) {
		let guard = REQUEST_CONFIG.get().unwrap().read().await;
		match OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.truncate(true)
			.open(&APP_CONFIG.get().unwrap().server_config.request_config_path)
			.await
		{
			Ok(mut client_config_file) => {
				if let Ok(client_config_buffer) = toml::to_string(&*guard) {
					if let Err(client_config_file_write_error) = client_config_file
						.write(&client_config_buffer.as_bytes())
						.await
					{
						error!(
							"unable to write to client config file: {}",
							client_config_file_write_error
						);
					}
				}
			}
			Err(client_config_file_error) => {
				error!(
					"unable to open client config file: {}",
					client_config_file_error
				);
			}
		}
	}
}
