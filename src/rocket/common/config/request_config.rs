use std::sync::OnceLock;

use serde_derive::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestConfig {
	/// The amount of time before a user can request another level
	pub cooldown_duration: u64,
	/// Allows levels to be requested, regardless of creator
	pub enable_requests: bool,
	/// Allows web requests to be made to GD servers, needed to power all GD
	/// related functionality
	pub enable_gd_requests: bool,
	/// Allows levels that are not uploaded by the requester to be requested
	pub allow_non_user_created_levels: bool,
}

pub static REQUEST_CONFIG: OnceLock<RwLock<RequestConfig>> = OnceLock::new();
