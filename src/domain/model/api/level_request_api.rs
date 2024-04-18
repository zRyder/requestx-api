use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::{DateTime, Local, Utc};
use rocket::serde::Serialize;
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde_derive::Deserialize;

use crate::{
	domain::model::{gd_level, gd_level::GDLevelRequest},
	rocket::common::constants::TIMESTAMP_HEADER_NAME
};

#[derive(Serialize)]
pub struct GetLevelRequestApiResponse {
	pub level_id: u64,
	pub discord_id: u64,
	pub discord_message_id: Option<u64>,
	pub discord_thread_id: Option<u64>,
	pub level_name: Option<String>,
	pub level_author: Option<String>,
	pub level_length: Option<LevelLength>,
	pub request_score: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool,
	pub timestamp: DateTime<Utc>
}

impl From<GDLevelRequest> for GetLevelRequestApiResponse {
	fn from(value: GDLevelRequest) -> Self {
		if let Some(gd_level) = value.gd_level {
			Self {
				level_id: value.level_id,
				discord_id: value.discord_user_id,
				discord_message_id: if let Some(message_data) = value.discord_message_data {
					Some(message_data.message_id)
				} else {
					None
				},
				discord_thread_id: if let Some(discord_message) = value.discord_message_data {
					if let Some(thread_id) = discord_message.thread_id {
						Some(thread_id)
					} else {
						None
					}
				} else {
					None
				},
				level_name: Some(gd_level.name),
				level_author: Some(gd_level.creator.name),
				level_length: Some(gd_level.level_length.into()),
				request_score: value.request_rating.into(),
				youtube_video_link: value.youtube_video_link,
				has_requested_feedback: value.has_requested_feedback,
				notify: value.notify,
				timestamp: value.timestamp
			}
		} else {
			Self {
				level_id: value.level_id,
				discord_id: value.discord_user_id,
				discord_message_id: if let Some(message_data) = value.discord_message_data {
					Some(message_data.message_id)
				} else {
					None
				},
				discord_thread_id: if let Some(discord_message) = value.discord_message_data {
					if let Some(thread_id) = discord_message.thread_id {
						Some(thread_id)
					} else {
						None
					}
				} else {
					None
				},
				level_name: None,
				level_author: None,
				level_length: None,
				request_score: value.request_rating.into(),
				youtube_video_link: value.youtube_video_link,
				has_requested_feedback: value.has_requested_feedback,
				notify: value.notify,
				timestamp: value.timestamp
			}
		}
	}
}

impl<'r> Responder<'r, 'r> for GetLevelRequestApiResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Ok)
			.raw_header("X-Timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Deserialize)]
pub struct PostLevelRequestApiRequest<'a> {
	pub level_id: u64,
	pub youtube_video_link: &'a str,
	pub discord_id: u64,
	pub request_rating: RequestRating,
	pub has_requested_feedback: bool,
	pub notify: bool
}

#[derive(Serialize)]
pub struct PostLevelRequestApiResponse {
	pub level_id: u64,
	pub discord_id: u64,
	pub level_name: Option<String>,
	pub level_author: Option<String>,
	pub level_length: Option<LevelLength>,
	pub request_score: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool
}

impl From<GDLevelRequest> for PostLevelRequestApiResponse {
	fn from(value: GDLevelRequest) -> Self {
		if let Some(gd_level) = value.gd_level {
			Self {
				level_id: value.level_id,
				discord_id: value.discord_user_id,
				level_name: Some(gd_level.name),
				level_author: Some(gd_level.creator.name),
				level_length: Some(gd_level.level_length.into()),
				request_score: value.request_rating.into(),
				youtube_video_link: value.youtube_video_link,
				has_requested_feedback: value.has_requested_feedback,
				notify: value.notify
			}
		} else {
			Self {
				level_id: value.level_id,
				discord_id: value.discord_user_id,
				level_name: None,
				level_author: None,
				level_length: None,
				request_score: value.request_rating.into(),
				youtube_video_link: value.youtube_video_link,
				has_requested_feedback: value.has_requested_feedback,
				notify: value.notify
			}
		}
	}
}

impl<'r> Responder<'r, 'r> for PostLevelRequestApiResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Created)
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Debug, PartialEq)]
pub enum LevelRequestApiResponseError {
	MalformedRequest,
	LevelRequestExists,
	LevelRequestDoesNotExist,
	UserOnCooldown,
	LevelRequetsDisabled,
	LevelRequestError
}

impl<'r> Responder<'r, 'r> for LevelRequestApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
		let json = Json(self.to_string());
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON);
		match self {
			LevelRequestApiResponseError::MalformedRequest => {
				response.status(Status::BadRequest);
			}
			LevelRequestApiResponseError::LevelRequestExists => {
				response.status(Status::Conflict);
			}
			LevelRequestApiResponseError::LevelRequestDoesNotExist => {
				response.status(Status::NotFound);
			}
			LevelRequestApiResponseError::UserOnCooldown => {
				response.status(Status::TooManyRequests);
			}
			LevelRequestApiResponseError::LevelRequetsDisabled => {
				response.status(Status::ServiceUnavailable);
			}
			LevelRequestApiResponseError::LevelRequestError => {
				response.status(Status::InternalServerError);
			}
		}

		response.ok()
	}
}

impl Display for LevelRequestApiResponseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelRequestApiResponseError::MalformedRequest => {
				write!(f, "{{\"message\": \"Level request was malformed\"}}")
			}
			LevelRequestApiResponseError::LevelRequestExists => {
				write!(f, "{{\"message\": \"Level has already been requested\"}}")
			}
			LevelRequestApiResponseError::LevelRequestDoesNotExist => {
				write!(f, "{{\"message\": \"Level request does not exist\"}}")
			}
			LevelRequestApiResponseError::UserOnCooldown => {
				write!(f, "{{\"message\": \"User is on cooldown\"}}")
			}
			LevelRequestApiResponseError::LevelRequetsDisabled => {
				write!(f, "{{\"message\": \"Level requests are disabled\"}}")
			}
			LevelRequestApiResponseError::LevelRequestError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}

impl Error for LevelRequestApiResponseError {}

#[derive(Deserialize, Serialize, Clone, Copy)]
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

impl Into<gd_level::RequestRating> for RequestRating {
	fn into(self) -> gd_level::RequestRating {
		match self {
			RequestRating::One => gd_level::RequestRating::One,
			RequestRating::Two => gd_level::RequestRating::Two,
			RequestRating::Three => gd_level::RequestRating::Three,
			RequestRating::Four => gd_level::RequestRating::Four,
			RequestRating::Five => gd_level::RequestRating::Five,
			RequestRating::Six => gd_level::RequestRating::Six,
			RequestRating::Seven => gd_level::RequestRating::Seven,
			RequestRating::Eight => gd_level::RequestRating::Eight,
			RequestRating::Nine => gd_level::RequestRating::Nine,
			RequestRating::Ten => gd_level::RequestRating::Ten
		}
	}
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum LevelLength {
	Tiny,
	Short,
	Medium,
	Long,
	ExtraLong,
	Platformer
}

impl Into<gd_level::LevelLength> for LevelLength {
	fn into(self) -> gd_level::LevelLength {
		match self {
			LevelLength::Tiny => gd_level::LevelLength::Tiny,
			LevelLength::Short => gd_level::LevelLength::Short,
			LevelLength::Medium => gd_level::LevelLength::Medium,
			LevelLength::Long => gd_level::LevelLength::Long,
			LevelLength::ExtraLong => gd_level::LevelLength::ExtraLong,
			LevelLength::Platformer => gd_level::LevelLength::Platformer
		}
	}
}
