use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::Responder,
	serde::json::Json,
	Request, Response
};

pub struct LevelRequestApiResponse {
	pub json: Json<String>,
	pub status: Status
}

impl<'r> Responder<'r, 'r> for LevelRequestApiResponse {
	fn respond_to(self, req: &Request) -> response::Result<'r> {
		Response::build_from(self.json.respond_to(&req).unwrap())
			.status(self.status)
			.raw_header("date", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}
