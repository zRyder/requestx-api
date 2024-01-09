use std::fmt::{Display, Formatter};

use chrono::Local;
use rocket::serde::Deserialize;
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::{Builder, Responder},
	serde::json::Json,
	Request, Response
};
use serde_derive::Serialize;

use crate::domain::model::{
	api::level_request_api::GetLevelRequestApiResponse, review::LevelReview
};

#[derive(Serialize)]
pub struct GetLevelReviewApiRespnse {
	pub level_id: u64,
	pub reviewer_discord_id: u64,
	pub discord_message_id: u64,
	pub review_contents: String
}

impl From<LevelReview> for GetLevelReviewApiRespnse {
	fn from(value: LevelReview) -> Self {
		Self {
			level_id: value.level_id,
			reviewer_discord_id: value.reviewer_discord_id,
			discord_message_id: value.discord_message_id,
			review_contents: value.review_contents
		}
	}
}

impl<'r> Responder<'r, 'r> for GetLevelReviewApiRespnse {
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
pub struct LevelReviewApiRequest<'a> {
	pub level_id: u64,
	pub reviewer_discord_id: u64,
	pub discord_message_id: u64,
	pub review_contents: &'a str
}

#[derive(Serialize)]
pub struct LevelReviewApiResponse {
	pub level_id: u64,
	pub reviewer_discord_id: u64,
	pub review_contents: String,
	pub is_update: bool
}

impl<'r> Responder<'r, 'r> for LevelReviewApiResponse {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(&self);
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header("x-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON);
		if self.is_update {
			response.status(Status::Ok);
			response.ok()
		} else {
			response.status(Status::Created);
			response.ok()
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum LevelReviewApiResponseError {
	LevelRequestDoesNotExist,
	LevelReviewError
}

impl<'r> Responder<'r, 'r> for LevelReviewApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
		let json = Json(self.to_string());
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header("x-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON);

		match self {
			LevelReviewApiResponseError::LevelRequestDoesNotExist => {
				response.status(Status::NotFound);
			}
			LevelReviewApiResponseError::LevelReviewError => {
				response.status(Status::InternalServerError);
			}
		}

		response.ok()
	}
}

impl Display for LevelReviewApiResponseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelReviewApiResponseError::LevelRequestDoesNotExist => {
				write!(f, "{{\"message\": \"The level request does not exist\"}}")
			}
			LevelReviewApiResponseError::LevelReviewError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}
