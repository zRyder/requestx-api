use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	serde::json::Json,
	Request, Response,
};
use serde_derive::{Deserialize, Serialize};

use crate::rocket::common::constants::TIMESTAMP_HEADER_NAME;

#[derive(Serialize)]
pub struct InternalGetRequestConfigApiResponse {
	#[serde(rename = "duration")]
	pub duration_in_minutes: u64,
	pub enable_requests: bool,
	pub enable_gd_requests: bool,
	pub allow_non_user_created_levels: bool,
}

impl<'r> Responder<'r, 'r> for InternalGetRequestConfigApiResponse {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Ok)
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Deserialize)]
pub struct InternalUpdateRequestConfigApiRequest {
	#[serde(rename = "duration")]
	pub duration_in_minutes: Option<u64>,
	pub enable_requests: Option<bool>,
	pub enable_gd_requests: Option<bool>,
	pub allow_non_user_created_levels: Option<bool>,
}

#[derive(Serialize)]
pub struct InternalUpdateRequestConfigApiResponse {}

impl<'r> Responder<'r, 'r> for InternalUpdateRequestConfigApiResponse {
	fn respond_to(self, _request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		Response::build()
			.status(Status::Ok)
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}
