use chrono::Duration;

use crate::rocket::common::config::client_config::{
	COOLDOWN_DURATION, ENABLE_GD_REQUESTS, ENABLE_REQUESTS
};

pub struct RequestManagerService {}

impl RequestManagerService {
	pub fn set_request_cooldown(&self, duration_in_minutes: u64) {
		let mut guard = COOLDOWN_DURATION.lock().unwrap();
		*guard = Duration::minutes(duration_in_minutes as i64);
		info!("Request cooldown set to {}", duration_in_minutes)
	}

	pub fn get_request_cooldown(&self) -> Duration {
		let guard = COOLDOWN_DURATION.lock().unwrap();
		*guard
	}

	pub fn set_enable_request(&self, enable_requests: bool) {
		let mut guard = ENABLE_REQUESTS.lock().unwrap();
		*guard = enable_requests;
		info!("Enable requests toggled to {}", enable_requests)
	}

	pub fn get_enable_request(&self) -> bool {
		let guard = ENABLE_REQUESTS.lock().unwrap();
		*guard
	}

	pub fn set_enable_gd_request(&self, enable_gd_requests: bool) {
		let mut guard = ENABLE_GD_REQUESTS.lock().unwrap();
		*guard = enable_gd_requests;
		info!("Enable GD requests toggled to {}", enable_gd_requests)
	}

	pub fn get_enable_gd_request(&self) -> bool {
		let guard = ENABLE_GD_REQUESTS.lock().unwrap();
		*guard
	}
}
