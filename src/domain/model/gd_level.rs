use dash_rs::model::level::online_level::ListedLevel;
use sea_orm::ActiveValue;

use crate::{
	adapter::mysql::model::{level_request, sea_orm_active_enums},
	domain::model::{level_creator::LevelCreator, request_rating::RequestRating}
};

#[derive(Clone)]
pub struct GDLevelRequest {
	pub level: GDLevel,
	pub request_rating: RequestRating
}

#[derive(Clone)]
pub struct GDLevel {
	pub level_id: u64,
	pub name: String,
	pub creator: LevelCreator,
	pub description: Option<String>
}

impl Into<level_request::ActiveModel> for GDLevelRequest {
	fn into(self) -> level_request::ActiveModel {
		level_request::ActiveModel {
			id: ActiveValue::Set(self.level.level_id),
			name: ActiveValue::Set(self.level.name),
			description: match self.level.description {
				Some(desc) => ActiveValue::Set(Some(desc)),
				None => ActiveValue::Set(None)
			},
			author: ActiveValue::Set(self.level.creator.name),
			request_rating: ActiveValue::Set(self.request_rating.into())
		}
	}
}

impl Into<sea_orm_active_enums::RequestRating> for RequestRating {
	fn into(self) -> sea_orm_active_enums::RequestRating {
		match self {
			RequestRating::Easy => sea_orm_active_enums::RequestRating::Easy,
			RequestRating::Normal => sea_orm_active_enums::RequestRating::Normal,
			RequestRating::Hard => sea_orm_active_enums::RequestRating::Hard,
			RequestRating::Harder => sea_orm_active_enums::RequestRating::Harder,
			RequestRating::Insane => sea_orm_active_enums::RequestRating::Insane,
			RequestRating::Demon => sea_orm_active_enums::RequestRating::Demon
		}
	}
}

impl From<&ListedLevel<'_>> for GDLevel {
	fn from(listed_level: &ListedLevel) -> Self {
		GDLevel {
			level_id: listed_level.level_id.clone(),
			name: listed_level.name.to_string(),
			creator: LevelCreator {
				name: listed_level.creator.clone().unwrap().name.to_string(),
				account_id: listed_level
					.creator
					.clone()
					.unwrap()
					.account_id
					.unwrap_or_default(),
				player_id: listed_level.creator.clone().unwrap().user_id
			},
			description: match &listed_level.description {
				Some(desc) => Some(desc.clone().into_processed().unwrap().0.to_string()),
				None => None
			}
		}
	}
}
