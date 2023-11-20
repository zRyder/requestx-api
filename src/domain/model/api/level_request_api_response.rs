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

pub struct LevelRequestApiResponse {}

impl<'r> Responder<'r, 'r> for LevelRequestApiResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		Response::build()
			.status(Status::Created)
			.raw_header("date", format!("{}", Local::now()))
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
		let response = response
			.raw_header("date", format!("{}", Local::now()))
			.header(ContentType::JSON);
		match self {
			LevelRequestApiResponseError::LevelRequestExists => {
				let response = response.status(Status::Conflict);
			}
			LevelRequestApiResponseError::LevelRequestError => {
				let response = response.status(Status::InternalServerError);
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
