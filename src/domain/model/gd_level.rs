use dash_rs::model::level::online_level::ListedLevel;
use sea_orm::ActiveValue;

use crate::{
	adapter::mysql::model::{level_request, sea_orm_active_enums},
	domain::model::{level_creator::LevelCreator, request_rating::RequestRating, user::User}
};
use crate::domain::model::api;

#[derive(Clone)]
pub struct GDLevelRequest {
	pub gd_level: GDLevel,
	pub discord_user: User,
	pub request_rating: RequestRating,
	pub youtube_video_link: String
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
			id: ActiveValue::Set(self.gd_level.level_id),
			discord_id: ActiveValue::set(self.discord_user.discord_id),
			name: ActiveValue::Set(self.gd_level.name),
			description: match self.gd_level.description {
				Some(desc) => ActiveValue::Set(Some(desc)),
				None => ActiveValue::Set(None)
			},
			author: ActiveValue::Set(self.gd_level.creator.name),
			request_rating: ActiveValue::Set(self.request_rating.into()),
			you_tube_video_link: ActiveValue::Set(self.youtube_video_link)
		}
	}
}

impl Into<sea_orm_active_enums::RequestRating> for RequestRating {
	fn into(self) -> sea_orm_active_enums::RequestRating {
		match self {
			RequestRating::One => sea_orm_active_enums::RequestRating::One,
			RequestRating::Two => sea_orm_active_enums::RequestRating::Two,
			RequestRating::Three => sea_orm_active_enums::RequestRating::Three,
			RequestRating::Four => sea_orm_active_enums::RequestRating::Four,
			RequestRating::Five => sea_orm_active_enums::RequestRating::Five,
			RequestRating::Six => sea_orm_active_enums::RequestRating::Six,
			RequestRating::Seven => sea_orm_active_enums::RequestRating::Seven,
			RequestRating::Eight => sea_orm_active_enums::RequestRating::Eight,
			RequestRating::Nine => sea_orm_active_enums::RequestRating::Nine,
			RequestRating::Ten => sea_orm_active_enums::RequestRating::Ten
		}
	}
}

impl Into<api::request_rating::RequestRating> for RequestRating {
	fn into(self) -> api::request_rating::RequestRating {
		match self {
			RequestRating::One => { api::request_rating::RequestRating::One }
			RequestRating::Two => { api::request_rating::RequestRating::Two }
			RequestRating::Three => { api::request_rating::RequestRating::Three }
			RequestRating::Four => { api::request_rating::RequestRating::Four }
			RequestRating::Five => { api::request_rating::RequestRating::Five }
			RequestRating::Six => { api::request_rating::RequestRating::Six }
			RequestRating::Seven => { api::request_rating::RequestRating::Seven }
			RequestRating::Eight => { api::request_rating::RequestRating::Eight }
			RequestRating::Nine => { api::request_rating::RequestRating::Nine }
			RequestRating::Ten => { api::request_rating::RequestRating::Ten }
		}
	}
}

impl From<&ListedLevel<'_>> for GDLevel {
	fn from(listed_level: &ListedLevel) -> Self {
		GDLevel {
			level_id: listed_level.level_id.clone(),
			name: listed_level.name.to_string(),
			creator: LevelCreator {
				name: listed_level.creator.as_ref().unwrap().name.to_string(),
				account_id: listed_level
					.creator
					.as_ref()
					.unwrap()
					.account_id
					.unwrap_or_default(),
				player_id: listed_level.creator.as_ref().unwrap().user_id
			},
			description: match &listed_level.description {
				Some(desc) => Some(desc.to_owned().into_processed().unwrap().0.to_string()),
				None => None
			}
		}
	}
}
