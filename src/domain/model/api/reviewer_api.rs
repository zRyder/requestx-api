use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde_derive::{Deserialize, Serialize};

use crate::{domain::model::reviewer::Reviewer, rocket::common::constants::TIMESTAMP_HEADER_NAME};

#[derive(Serialize)]
pub struct GetReviewerApiResponse {
	pub reviewer_discord_id: u64,
	pub is_active: bool
}

#[derive(Deserialize)]
pub struct CreateReviewerApiRequest {
	pub reviewer_discord_id: u64
}

impl From<Reviewer> for GetReviewerApiResponse {
	fn from(value: Reviewer) -> Self {
		Self {
			reviewer_discord_id: value.discord_id,
			is_active: value.is_active
		}
	}
}

impl<'r> Responder<'r, 'r> for GetReviewerApiResponse {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Ok)
			.raw_header("X-Timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Debug, PartialEq)]
pub enum ReviewerApiResponseError {
	ReviewerDoesNotExist,
	ReviewerError
}

impl<'r> Responder<'r, 'r> for ReviewerApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(self.to_string());
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON);
		match self {
			ReviewerApiResponseError::ReviewerDoesNotExist => {
				response.status(Status::NotFound);
			}
			ReviewerApiResponseError::ReviewerError => {
				response.status(Status::InternalServerError);
			}
		}

		response.ok()
	}
}

impl Display for ReviewerApiResponseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ReviewerApiResponseError::ReviewerDoesNotExist => {
				write!(f, "{{\"message\": \"Reviewer does not exist\"}}")
			}
			ReviewerApiResponseError::ReviewerError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}

impl Error for ReviewerApiResponseError {}
