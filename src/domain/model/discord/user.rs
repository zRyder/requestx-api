use sea_orm::ActiveValue;

use crate::adapter::mysql::model::user;

#[derive(Debug, Clone, Copy)]
pub struct DiscordUser {
	pub discord_user_id: u64
}

impl Into<user::ActiveModel> for DiscordUser {
	fn into(self) -> user::ActiveModel {
		user::ActiveModel {
			discord_id: ActiveValue::Set(self.discord_user_id)
		}
	}
}
