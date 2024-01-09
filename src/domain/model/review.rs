use sea_orm::ActiveValue;

use crate::adapter::mysql::model::{review, review::Model};

#[derive(Debug, Clone)]
pub struct LevelReview {
	pub reviewer_discord_id: u64,
	pub discord_message_id: u64,
	pub level_id: u64,
	pub review_contents: String,
	pub is_update: bool
}

impl Into<review::ActiveModel> for LevelReview {
	fn into(self) -> review::ActiveModel {
		review::ActiveModel {
			level_id: ActiveValue::Set(self.level_id),
			discord_id: ActiveValue::Set(self.reviewer_discord_id),
			message_id: ActiveValue::Set(self.discord_message_id),
			review_content: ActiveValue::Set(self.review_contents)
		}
	}
}

impl From<Model> for LevelReview {
	fn from(value: Model) -> Self {
		Self {
			reviewer_discord_id: value.discord_id,
			discord_message_id: value.message_id,
			level_id: value.level_id,
			review_contents: value.review_content,
			is_update: false
		}
	}
}
