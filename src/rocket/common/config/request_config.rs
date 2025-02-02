use std::sync::OnceLock;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestConfig {
	pub cooldown_duration: u64,
	pub enable_requests: bool,
	pub enable_gd_requests: bool
}

pub static REQUEST_CONFIG: OnceLock<RwLock<RequestConfig>> = OnceLock::new();