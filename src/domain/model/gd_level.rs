use dash_rs::model::level::{online_level::ListedLevel, LevelLength as DashrsLevelLength};
use sea_orm::ActiveValue;

use crate::{
	adapter::mysql::model::{level_request, level_request::Model, sea_orm_active_enums},
	domain::model::{
		api,
		discord::{message::DiscordMessage, user::DiscordUser}
	}
};

#[derive(Clone)]
pub struct GDLevelRequest {
	pub gd_level: GDLevel,
	pub discord_user_data: DiscordUser,
	pub discord_message_data: Option<DiscordMessage>,
	pub request_rating: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool
}

#[derive(Clone)]
pub struct GDLevel {
	pub level_id: u64,
	pub name: String,
	pub creator: LevelCreator,
	pub level_length: LevelLength
}

#[derive(Clone)]
pub struct LevelCreator {
	pub name: String,
	pub account_id: u64,
	pub player_id: u64
}

impl Into<level_request::ActiveModel> for GDLevelRequest {
	fn into(self) -> level_request::ActiveModel {
		level_request::ActiveModel {
			level_id: ActiveValue::Set(self.gd_level.level_id),
			discord_id: ActiveValue::set(self.discord_user_data.discord_user_id),
			discord_message_id: ActiveValue::Set(
				if let Some(discord_message) = self.discord_message_data {
					Some(discord_message.message_id)
				} else {
					None
				}
			),
			discord_thread_id: ActiveValue::Set(
				if let Some(discord_message) = self.discord_message_data {
					if let Some(thread_id) = discord_message.thread_id {
						Some(thread_id)
					} else {
						None
					}
				} else {
					None
				}
			),
			name: ActiveValue::Set(self.gd_level.name),
			level_length: ActiveValue::Set(self.gd_level.level_length.into()),
			author: ActiveValue::Set(self.gd_level.creator.name),
			request_rating: ActiveValue::Set(self.request_rating.into()),
			you_tube_video_link: ActiveValue::Set(self.youtube_video_link),
			has_requested_feedback: ActiveValue::Set(self.has_requested_feedback.into()),
			notify: ActiveValue::Set(self.notify.into())
		}
	}
}

impl From<Model> for GDLevelRequest {
	fn from(value: Model) -> Self {
		Self {
			gd_level: GDLevel {
				level_id: value.level_id,
				name: value.name,
				creator: LevelCreator {
					name: value.author,
					account_id: 0,
					player_id: 0
				},
				level_length: LevelLength::from(value.level_length)
			},
			discord_user_data: DiscordUser {
				discord_user_id: value.discord_id
			},
			discord_message_data: if let Some(message_id) = value.discord_message_id {
				Some(DiscordMessage {
					message_id,
					thread_id: if let Some(thread_id) = value.discord_thread_id {
						Some(thread_id)
					} else {
						None
					}
				})
			} else {
				None
			},
			request_rating: RequestRating::from(value.request_rating),
			youtube_video_link: value.you_tube_video_link,
			has_requested_feedback: if value.has_requested_feedback != 0 {
				true
			} else {
				false
			},
			notify: if value.notify != 0 { true } else { false }
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
			level_length: LevelLength::from(listed_level.length)
		}
	}
}

#[derive(Clone, Copy)]
pub enum LevelLength {
	Tiny,
	Short,
	Medium,
	Long,
	ExtraLong,
	Platformer
}

impl From<sea_orm_active_enums::LevelLength> for LevelLength {
	fn from(value: sea_orm_active_enums::LevelLength) -> Self {
		match value {
			sea_orm_active_enums::LevelLength::Tiny => Self::Tiny,
			sea_orm_active_enums::LevelLength::Short => Self::Short,
			sea_orm_active_enums::LevelLength::Medium => Self::Medium,
			sea_orm_active_enums::LevelLength::Long => Self::Long,
			sea_orm_active_enums::LevelLength::ExtraLong => Self::ExtraLong,
			sea_orm_active_enums::LevelLength::Platformer => Self::Platformer
		}
	}
}

impl Into<sea_orm_active_enums::LevelLength> for LevelLength {
	fn into(self) -> sea_orm_active_enums::LevelLength {
		match self {
			LevelLength::Tiny => sea_orm_active_enums::LevelLength::Tiny,
			LevelLength::Short => sea_orm_active_enums::LevelLength::Short,
			LevelLength::Medium => sea_orm_active_enums::LevelLength::Medium,
			LevelLength::Long => sea_orm_active_enums::LevelLength::Long,
			LevelLength::ExtraLong => sea_orm_active_enums::LevelLength::ExtraLong,
			LevelLength::Platformer => sea_orm_active_enums::LevelLength::Platformer
		}
	}
}

impl From<DashrsLevelLength> for LevelLength {
	fn from(value: DashrsLevelLength) -> Self {
		match value {
			DashrsLevelLength::Unknown(_) => Self::Tiny,
			DashrsLevelLength::Tiny => Self::Tiny,
			DashrsLevelLength::Short => Self::Short,
			DashrsLevelLength::Medium => Self::Medium,
			DashrsLevelLength::Long => Self::Long,
			DashrsLevelLength::ExtraLong => Self::ExtraLong,
			DashrsLevelLength::Platformer => Self::Platformer
		}
	}
}

impl Into<api::level_request_api::LevelLength> for LevelLength {
	fn into(self) -> api::level_request_api::LevelLength {
		match self {
			LevelLength::Tiny => api::level_request_api::LevelLength::Tiny,
			LevelLength::Short => api::level_request_api::LevelLength::Short,
			LevelLength::Medium => api::level_request_api::LevelLength::Medium,
			LevelLength::Long => api::level_request_api::LevelLength::Long,
			LevelLength::ExtraLong => api::level_request_api::LevelLength::ExtraLong,
			LevelLength::Platformer => api::level_request_api::LevelLength::Platformer
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum RequestRating {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten
}

impl From<sea_orm_active_enums::RequestRating> for RequestRating {
	fn from(value: sea_orm_active_enums::RequestRating) -> Self {
		match value {
			sea_orm_active_enums::RequestRating::One => Self::One,
			sea_orm_active_enums::RequestRating::Two => Self::Two,
			sea_orm_active_enums::RequestRating::Three => Self::Three,
			sea_orm_active_enums::RequestRating::Four => Self::Four,
			sea_orm_active_enums::RequestRating::Five => Self::Five,
			sea_orm_active_enums::RequestRating::Six => Self::Six,
			sea_orm_active_enums::RequestRating::Seven => Self::Seven,
			sea_orm_active_enums::RequestRating::Eight => Self::Eight,
			sea_orm_active_enums::RequestRating::Nine => Self::Nine,
			sea_orm_active_enums::RequestRating::Ten => Self::Ten
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

impl Into<api::level_request_api::RequestRating> for RequestRating {
	fn into(self) -> api::level_request_api::RequestRating {
		match self {
			RequestRating::One => api::level_request_api::RequestRating::One,
			RequestRating::Two => api::level_request_api::RequestRating::Two,
			RequestRating::Three => api::level_request_api::RequestRating::Three,
			RequestRating::Four => api::level_request_api::RequestRating::Four,
			RequestRating::Five => api::level_request_api::RequestRating::Five,
			RequestRating::Six => api::level_request_api::RequestRating::Six,
			RequestRating::Seven => api::level_request_api::RequestRating::Seven,
			RequestRating::Eight => api::level_request_api::RequestRating::Eight,
			RequestRating::Nine => api::level_request_api::RequestRating::Nine,
			RequestRating::Ten => api::level_request_api::RequestRating::Ten
		}
	}
}
