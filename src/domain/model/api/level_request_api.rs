use std::{
	error::Error,
	fmt::{Debug, Display, Formatter}
};

use chrono::{DateTime, Duration, Local, Utc};
use rocket::serde::Serialize;
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde::{ser::SerializeStruct, Serializer};
use serde_derive::Deserialize;

use crate::{
	domain::model::{
		internal::api::moderator_api::{SuggestedRating, SuggestedScore},
		level_request,
		level_request::LevelRequest,
		moderator::Moderator
	},
	rocket::common::constants::TIMESTAMP_HEADER_NAME
};

#[derive(Serialize)]
pub struct GetLevelRequestApiResponse {
	pub level_id: u64,
	pub discord_id: u64,
	pub discord_message_id: Option<u64>,
	pub level_name: Option<String>,
	pub level_author: Option<String>,
	pub level_length: Option<LevelLength>,
	pub request_rating: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool,
	pub timestamp: DateTime<Utc>
}

impl From<LevelRequest> for GetLevelRequestApiResponse {
	fn from(value: LevelRequest) -> Self {
		if let Some(gd_level) = value.gd_level {
			Self {
				level_id: value.level_id,
				discord_id: value.discord_user_id,
				discord_message_id: if let Some(message_data) = value.discord_message_data {
					Some(message_data.message_id)
				} else {
					None
				},
				level_name: Some(gd_level.name),
				level_author: Some(gd_level.creator.name),
				level_length: Some(gd_level.level_length.into()),
				request_rating: value.request_rating.into(),
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
				level_name: None,
				level_author: None,
				level_length: None,
				request_rating: value.request_rating.into(),
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

#[derive(Serialize)]
pub struct PostSendLevelRequestApiResponse {
	pub level_request: GetLevelRequestApiResponse,
	pub moderator_data: ModeratorDataApiResponse
}

impl From<(LevelRequest, Moderator)> for PostSendLevelRequestApiResponse {
	fn from(value: (LevelRequest, Moderator)) -> Self {
		if let Some(gd_level) = value.0.gd_level {
			Self {
				level_request: GetLevelRequestApiResponse {
					level_id: value.0.level_id,
					discord_id: value.0.discord_user_id,
					discord_message_id: if let Some(message_data) = value.0.discord_message_data {
						Some(message_data.message_id)
					} else {
						None
					},
					level_name: Some(gd_level.name),
					level_author: Some(gd_level.creator.name),
					level_length: Some(gd_level.level_length.into()),
					request_rating: value.0.request_rating.into(),
					youtube_video_link: value.0.youtube_video_link,
					has_requested_feedback: value.0.has_requested_feedback,
					notify: value.0.notify,
					timestamp: value.0.timestamp
				},
				moderator_data: ModeratorDataApiResponse {
					suggested_score: value.1.suggested_score.into(),
					suggested_rating: value.1.suggested_rating.into()
				}
			}
		} else {
			Self {
				level_request: GetLevelRequestApiResponse {
					level_id: value.0.level_id,
					discord_id: value.0.discord_user_id,
					discord_message_id: if let Some(message_data) = value.0.discord_message_data {
						Some(message_data.message_id)
					} else {
						None
					},
					level_name: None,
					level_author: None,
					level_length: None,
					request_rating: value.0.request_rating.into(),
					youtube_video_link: value.0.youtube_video_link,
					has_requested_feedback: value.0.has_requested_feedback,
					notify: value.0.notify,
					timestamp: value.0.timestamp
				},
				moderator_data: ModeratorDataApiResponse {
					suggested_score: value.1.suggested_score.into(),
					suggested_rating: value.1.suggested_rating.into()
				}
			}
		}
	}
}

impl<'r> Responder<'r, 'r> for PostSendLevelRequestApiResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Ok)
			.raw_header("X-Timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Serialize)]
pub struct ModeratorDataApiResponse {
	pub suggested_score: SuggestedScore,
	pub suggested_rating: SuggestedRating
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
	pub request_rating: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool
}

#[derive(Deserialize)]
pub struct PatchLevelRequestApiRequest<'a> {
	pub level_id: u64,
	pub discord_id: u64,
	pub youtube_video_link: Option<&'a str>,
	pub request_rating: Option<RequestRating>,
	pub has_requested_feedback: Option<bool>,
	pub notify: Option<bool>
}

impl From<LevelRequest> for PostLevelRequestApiResponse {
	fn from(value: LevelRequest) -> Self {
		if let Some(gd_level) = value.gd_level {
			Self {
				level_id: value.level_id,
				discord_id: value.discord_user_id,
				level_name: Some(gd_level.name),
				level_author: Some(gd_level.creator.name),
				level_length: Some(gd_level.level_length.into()),
				request_rating: value.request_rating.into(),
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
				request_rating: value.request_rating.into(),
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
	UserOnCooldown(DateTime<Utc>, Duration),
	RequestNonCreatedLevel,
	EditUnownedLevelRequest(u64, u64, u64),
	LevelRequestDisabled,
	LevelRequestError
}

impl Serialize for LevelRequestApiResponseError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		let mut state = serializer.serialize_struct("LevelRequestApiResponseError", 3)?;

		state.serialize_field("message", &self.to_string())?;

		match self {
			LevelRequestApiResponseError::UserOnCooldown(last_request_time, request_cooldown) => {
				state.serialize_field("last_request_time", last_request_time)?;
				state.serialize_field("request_cooldown", &request_cooldown.num_minutes())?;
			}
			_ => {}
		}

		state.end()
	}
}

impl<'r> Responder<'r, 'r> for LevelRequestApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
		let json = Json(&self);
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
			LevelRequestApiResponseError::UserOnCooldown(_, _) => {
				response.status(Status::TooManyRequests);
			}
			LevelRequestApiResponseError::RequestNonCreatedLevel => {
				response.status(Status::BadRequest);
			}
			LevelRequestApiResponseError::EditUnownedLevelRequest(_, _, _) => {
				response.status(Status::Forbidden);
			}
			LevelRequestApiResponseError::LevelRequestDisabled => {
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
				write!(f, "Level request was malformed")
			}
			LevelRequestApiResponseError::LevelRequestExists => {
				write!(f, "Level has already been requested")
			}
			LevelRequestApiResponseError::LevelRequestDoesNotExist => {
				write!(f, "Level request does not exist")
			}
			LevelRequestApiResponseError::UserOnCooldown(_, _) => {
				write!(f, "User is on cooldown")
			}
			LevelRequestApiResponseError::EditUnownedLevelRequest(_, _, _) => {
				write!(f, "User attempted to edit a request they do not own")
			}
			LevelRequestApiResponseError::RequestNonCreatedLevel => {
				write!(f, "User attempted to request a level they did not create")
			}
			LevelRequestApiResponseError::LevelRequestDisabled => {
				write!(f, "Level requests are disabled")
			}
			LevelRequestApiResponseError::LevelRequestError => {
				write!(f, "Internal server error")
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

impl Into<level_request::RequestRating> for RequestRating {
	fn into(self) -> level_request::RequestRating {
		match self {
			RequestRating::One => level_request::RequestRating::One,
			RequestRating::Two => level_request::RequestRating::Two,
			RequestRating::Three => level_request::RequestRating::Three,
			RequestRating::Four => level_request::RequestRating::Four,
			RequestRating::Five => level_request::RequestRating::Five,
			RequestRating::Six => level_request::RequestRating::Six,
			RequestRating::Seven => level_request::RequestRating::Seven,
			RequestRating::Eight => level_request::RequestRating::Eight,
			RequestRating::Nine => level_request::RequestRating::Nine,
			RequestRating::Ten => level_request::RequestRating::Ten
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

impl Into<level_request::LevelLength> for LevelLength {
	fn into(self) -> level_request::LevelLength {
		match self {
			LevelLength::Tiny => level_request::LevelLength::Tiny,
			LevelLength::Short => level_request::LevelLength::Short,
			LevelLength::Medium => level_request::LevelLength::Medium,
			LevelLength::Long => level_request::LevelLength::Long,
			LevelLength::ExtraLong => level_request::LevelLength::ExtraLong,
			LevelLength::Platformer => level_request::LevelLength::Platformer
		}
	}
}
