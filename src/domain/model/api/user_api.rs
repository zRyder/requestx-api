use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::{DateTime, Duration, Local, Utc};
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::{
	domain::{
		model::discord::user::DiscordUser,
		service::internal::request_manager_service::RequestManagerService
	},
	rocket::common::constants::TIMESTAMP_HEADER_NAME
};

pub struct GetDiscordUserApiResponse {
	pub discord_user_id: u64,
	pub last_request_time: Option<DateTime<Utc>>,
	pub request_cooldown: Duration
}

impl Serialize for GetDiscordUserApiResponse {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		let mut state = serializer.serialize_struct("LevelRequestApiResponseError", 3)?;
		state.serialize_field("discord_user_id", &self.discord_user_id)?;
		state.serialize_field("last_request_time", &self.last_request_time)?;
		state.serialize_field("request_cooldown", &self.request_cooldown.num_minutes())?;

		state.end()
	}
}

impl From<DiscordUser> for GetDiscordUserApiResponse {
	fn from(value: DiscordUser) -> Self {
		Self {
			discord_user_id: value.discord_user_id,
			last_request_time: value.last_request_time,
			request_cooldown: RequestManagerService {}.get_request_cooldown()
		}
	}
}

impl<'r> Responder<'r, 'r> for GetDiscordUserApiResponse {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(self);
		Response::build_from(json.respond_to(&request).unwrap())
			.status(Status::Ok)
			.raw_header("X-Timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.ok()
	}
}

#[derive(Debug, PartialEq, Serialize)]
pub enum DiscordUserApiResponseError {
	UserDoesNotExist,
	DiscordUserError
}

impl<'r> Responder<'r, 'r> for DiscordUserApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(&self);
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON);

		match self {
			DiscordUserApiResponseError::UserDoesNotExist => {
				response.status(Status::NotFound);
			}
			DiscordUserApiResponseError::DiscordUserError => {
				response.status(Status::InternalServerError);
			}
		}

		response.ok()
	}
}

impl Display for DiscordUserApiResponseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DiscordUserApiResponseError::UserDoesNotExist => {
				write!(f, "User does not exist")
			}
			DiscordUserApiResponseError::DiscordUserError => {
				write!(f, "Internal server error")
			}
		}
	}
}

impl Error for DiscordUserApiResponseError {}
