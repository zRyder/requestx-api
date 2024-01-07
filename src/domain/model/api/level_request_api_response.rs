use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::Responder,
	serde::json::Json,
	Request, Response
};
use rocket::serde::Serialize;
use crate::domain::model::api::request_rating::RequestRating;

#[derive(Serialize)]
pub struct LevelRequestApiResponse {
	pub level_id: u64,
	pub discord_id: u64,
	pub level_name: String,
	pub level_author: String,
	pub request_score: RequestRating,
	pub youtube_video_link: String
}

impl<'r> Responder<'r, 'r> for LevelRequestApiResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		let json =  Json(self);
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
			LevelRequestApiResponseError::LevelRequestError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}

impl Error for LevelRequestApiResponseError {}
