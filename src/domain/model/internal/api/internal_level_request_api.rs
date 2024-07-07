use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	Request, Response
};
use serde_derive::{Deserialize, Serialize};

use crate::rocket::common::constants::TIMESTAMP_HEADER_NAME;

#[derive(Deserialize)]
pub struct InternalUpdateLevelRequestMessageIdApiRequest {
	pub level_id: u64,
	pub discord_message_id: u64
}

#[derive(Serialize)]
pub struct InternalUpdateLevelRequestDiscordDataApiResponse {}

impl<'r> Responder<'r, 'r> for InternalUpdateLevelRequestDiscordDataApiResponse {
	fn respond_to(self, _request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		Response::build()
			.status(Status::Ok)
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}
