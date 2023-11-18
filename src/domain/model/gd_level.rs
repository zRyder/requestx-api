use sea_orm::ActiveValue;

use crate::{
	adapter::mysql::model::{level_request, sea_orm_active_enums},
	domain::model::{level_creator::LevelCreator, request_rating::RequestRating}
};

#[derive(Clone)]
pub struct GDLevel {
	pub level_id: u64,
	pub name: String,
	pub creator: LevelCreator,
	pub description: Option<String>,
	pub request_rating: RequestRating
}

impl Into<level_request::ActiveModel> for GDLevel {
	fn into(self) -> level_request::ActiveModel {
		level_request::ActiveModel {
			id: ActiveValue::Set(self.level_id),
			name: ActiveValue::Set(self.name),
			description: match self.description {
				Some(desc) => ActiveValue::Set(Some(desc)),
				None => ActiveValue::Set(None)
			},
			author: ActiveValue::Set(self.creator.name),
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
