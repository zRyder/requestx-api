use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	Request, Response
};
use serde_derive::{Deserialize, Serialize};

use crate::rocket::common::constants::TIMESTAMP_HEADER_NAME;

#[derive(Deserialize)]
pub struct InternalUpdateRequestConfigApiRequest {
	#[serde(rename = "duration")]
	pub duration_in_minutes: Option<u64>,
	pub enable_requests: Option<bool>
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
