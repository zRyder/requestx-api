use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;

use crate::adapter::mysql::model::{user, user::Model};

#[derive(Debug, Clone)]
pub struct DiscordUser {
	pub discord_user_id: u64,
	pub gd_player_id: Option<u64>,
	pub last_request_time: Option<DateTime<Utc>>,
	pub gd_account_hash: Option<String>
}

impl Into<user::ActiveModel> for DiscordUser {
	fn into(self) -> user::ActiveModel {
		user::ActiveModel {
			discord_id: ActiveValue::Set(self.discord_user_id),
			timestamp: ActiveValue::Set(if let Some(last_request_time) = self.last_request_time {
				Some(last_request_time)
			} else {
				None
			}),
			gd_account_hash: ActiveValue::Set(self.gd_account_hash),
			gd_player_id: ActiveValue::Set(self.gd_player_id)
		}
	}
}

impl From<Model> for DiscordUser {
	fn from(value: Model) -> Self {
		Self {
			discord_user_id: value.discord_id,
			gd_player_id: value.gd_player_id,
			last_request_time: value.timestamp,
			gd_account_hash: value.gd_account_hash
		}
	}
}
