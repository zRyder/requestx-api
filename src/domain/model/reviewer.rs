use sea_orm::ActiveValue;

use crate::adapter::mysql::model::reviewer;

#[derive(Copy, Clone, Debug)]
pub struct Reviewer {
	pub discord_id: u64,
	pub is_active: bool
}

impl Into<reviewer::ActiveModel> for Reviewer {
	fn into(self) -> reviewer::ActiveModel {
		reviewer::ActiveModel {
			discord_id: ActiveValue::Set(self.discord_id),
			active: ActiveValue::Set(i8::from(self.is_active))
		}
	}
}

impl From<reviewer::Model> for Reviewer {
	fn from(value: reviewer::Model) -> Self {
		Self {
			discord_id: value.discord_id,
			is_active: if value.active != 0 { true } else { false }
		}
	}
}
