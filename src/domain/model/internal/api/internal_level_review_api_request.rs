use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	Request, Response
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InternalUpdateLevelReviewMessageIdApiRequest {
	pub level_id: u64,
	pub discord_id: u64,
	pub discord_message_id: u64
}

#[derive(Serialize)]
pub struct InternalUpdateLevelReviewDiscordDataApiResponse {}

impl<'r> Responder<'r, 'r> for InternalUpdateLevelReviewDiscordDataApiResponse {
	fn respond_to(self, _request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		Response::build()
			.status(Status::Ok)
			.raw_header("x-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}
