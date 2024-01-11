use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::Local;
use rocket::serde::Serialize;
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde_derive::Deserialize;

use crate::domain::model::{
	api::request_rating::RequestRating,
	gd_level::GDLevelRequest
};

#[derive(Serialize)]
pub struct GetLevelRequestApiResponse {
	pub level_id: u64,
	pub discord_id: u64,
	pub discord_message_id: Option<u64>,
	pub discord_thread_id: Option<u64>,
	pub level_name: String,
	pub level_author: String,
	pub request_score: RequestRating,
	pub youtube_video_link: String
}

impl From<GDLevelRequest> for GetLevelRequestApiResponse {
	fn from(value: GDLevelRequest) -> Self {
		Self {
			level_id: value.gd_level.level_id,
			discord_id: value.discord_user_data.discord_user_id,
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
			level_name: value.gd_level.name,
			level_author: value.gd_level.creator.name,
			request_score: value.request_rating.into(),
			youtube_video_link: value.youtube_video_link
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
	pub request_rating: RequestRating
}

#[derive(Serialize)]
pub struct PostLevelRequestApiResponse {
	pub level_id: u64,
	pub discord_id: u64,
	pub level_name: String,
	pub level_author: String,
	pub request_score: RequestRating,
	pub youtube_video_link: String
}

impl From<GDLevelRequest> for PostLevelRequestApiResponse {
	fn from(value: GDLevelRequest) -> Self {
		Self {
			level_id: value.gd_level.level_id,
			discord_id: value.discord_user_data.discord_user_id,
			level_name: value.gd_level.name,
			level_author: value.gd_level.creator.name,
			request_score: value.request_rating.into(),
			youtube_video_link: value.youtube_video_link
		}
	}
}

impl<'r> Responder<'r, 'r> for PostLevelRequestApiResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Created)
			.raw_header("x-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Debug, PartialEq)]
pub enum LevelRequestApiResponseError {
	LevelRequestExists,
	LevelRequestDoesNotExist,
	LevelRequestError
}

impl<'r> Responder<'r, 'r> for LevelRequestApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
		let json = Json(self.to_string());
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header("x-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON);
		match self {
			LevelRequestApiResponseError::LevelRequestExists => {
				response.status(Status::Conflict);
			}
			LevelRequestApiResponseError::LevelRequestDoesNotExist => {
				response.status(Status::NotFound);
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
			LevelRequestApiResponseError::LevelRequestExists => {
				write!(f, "{{\"message\": \"Level has already been requested\"}}")
			}
			LevelRequestApiResponseError::LevelRequestDoesNotExist => {
				write!(f, "{{\"message\": \"Level request does not exist\"}}")
			}
			LevelRequestApiResponseError::LevelRequestError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}

impl Error for LevelRequestApiResponseError {}
