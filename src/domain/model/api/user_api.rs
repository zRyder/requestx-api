use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::{DateTime, Duration, Local, Utc};
use rocket_framework::{
	http::{ContentType, Status},
	response,
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_derive::Deserialize;
use tokio::runtime::Runtime;

use crate::{
	domain::{
		model::discord::user::{DiscordGDAccountLink, DiscordUser},
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
			request_cooldown: Runtime::new()
				.unwrap()
				.block_on(RequestManagerService {}.get_request_cooldown())
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
	GDAccountDoesNotExist(String),
	GDAccountLinkExpired,
	InvalidGDAccountLinkToken,
	DiscordAccountAlreadyLinked,
	DiscordUserError
}

impl<'r> Responder<'r, 'r> for DiscordUserApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
		let json = Json(&self);
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.header(ContentType::JSON);

		match self {
			DiscordUserApiResponseError::UserDoesNotExist => {
				response.status(Status::NotFound);
			}
			DiscordUserApiResponseError::GDAccountDoesNotExist(_) => {
				response.status(Status::NotFound);
			}
			DiscordUserApiResponseError::GDAccountLinkExpired => {
				response.status(Status::Gone);
			}
			DiscordUserApiResponseError::DiscordAccountAlreadyLinked => {
				response.status(Status::Conflict);
			}
			DiscordUserApiResponseError::InvalidGDAccountLinkToken => {
				response.status(Status::Unauthorized);
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
			DiscordUserApiResponseError::GDAccountDoesNotExist(gd_username) => {
				write!(
					f,
					"Geometry Dash user does not exist with username {}",
					gd_username
				)
			}
			DiscordUserApiResponseError::GDAccountLinkExpired => {
				write!(f, "GD account link has expired")
			}
			DiscordUserApiResponseError::InvalidGDAccountLinkToken => {
				write!(f, "GD account link token was invalid")
			}
			DiscordUserApiResponseError::DiscordAccountAlreadyLinked => {
				write!(f, "Discord Account link is already linked to a GD account")
			}
			DiscordUserApiResponseError::DiscordUserError => {
				write!(f, "Internal server error")
			}
		}
	}
}

impl Error for DiscordUserApiResponseError {}

#[derive(Deserialize)]
pub struct PostLinkGDAccountRequest<'a> {
	pub discord_id: u64,
	pub gd_username: &'a str
}

#[derive(Serialize)]
pub struct PostLinkGDAccountResponse {
	pub discord_id: u64,
	pub gd_username: String,
	pub gd_player_id: u64,
	#[serde(skip_serializing)]
	pub gd_account_requestx_token: String
}

impl<'r> Responder<'r, 'r> for PostLinkGDAccountResponse {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		let gd_account_requestx_token = self.gd_account_requestx_token.clone();
		let json = Json(self);
		Response::build_from(json.respond_to(&request)?)
			.status(Status::Created)
			.raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
			.raw_header("X-RequestX-GD-Link-Token", gd_account_requestx_token)
			.header(ContentType::JSON)
			.ok()
	}
}

impl From<DiscordGDAccountLink> for PostLinkGDAccountResponse {
	fn from(value: DiscordGDAccountLink) -> Self {
		Self {
			discord_id: value.discord_user_id,
			gd_player_id: value.gd_player_id,
			gd_username: value.gd_username,
			gd_account_requestx_token: value.gd_account_challenge
		}
	}
}
