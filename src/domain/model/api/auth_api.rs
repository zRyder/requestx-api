use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::Local;
use jsonwebtoken::{decode, errors::Error as JsonWebTokenError, DecodingKey, Validation};
use rocket::serde::{Deserialize, Serialize};
use rocket_framework::{
	http::{ContentType, Status},
	request::{FromRequest, Outcome},
	response::Responder,
	serde::json::Json,
	Request, Response
};

use crate::{
	domain::model::{auth::claims::Claims, error::level_request_error::LevelRequestError},
	rocket::common::config::auth_config::AUTH_CONFIG
};
use crate::rocket::common::config::client_config::CLIENT_CONFIG;

#[derive(Deserialize)]
pub struct AuthApiRequest {
	pub discord_app_id: u64,
	_access_token: String
}

#[derive(Serialize)]
pub struct AuthApiResponse {
	jwt: String
}

#[derive(Deserialize)]
pub struct Auth {}

#[derive(Debug, PartialEq)]
pub enum AuthApiError {
	AuthError
}

impl AuthApiResponse {
	pub fn new(jwt: String) -> Self { Self { jwt } }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthApiRequest {
	type Error = LevelRequestError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let discord_app_id = request.headers().get_one("X-REQUESTX-DISCORD-APP-ID");
		let access_token = request.headers().get_one("X-REQUESTX-ACCESS-TOKEN");
		if discord_app_id.is_some() && access_token.is_some() {
			if discord_app_id.unwrap().parse::<u64>().unwrap().ne(&CLIENT_CONFIG.discord_app_id) {
				Outcome::Forward(Status::Unauthorized)
			} else if access_token.unwrap().ne(&AUTH_CONFIG.access_token) {
				Outcome::Forward(Status::Forbidden)
			} else {
				Outcome::Success(AuthApiRequest {
					discord_app_id: discord_app_id.unwrap().parse::<u64>().unwrap(),
					_access_token: access_token.unwrap().to_owned()
				})
			}
		} else {
			Outcome::Forward(Status::Unauthorized)
		}
	}
}

impl<'r> Responder<'r, 'r> for AuthApiResponse {
	fn respond_to(self, _request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		Response::build()
			.status(Status::Created)
			.raw_header("AUTHORIZATION", self.jwt)
			.raw_header("X-TIMESTAMP", format!("{}", Local::now()))
			.ok()
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
	type Error = ();

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let discord_app_id = request.headers().get_one("X-REQUESTX-DISCORD-APP-ID");
		let jwt = request.headers().get_one("AUTHORIZATION");
		if discord_app_id.is_some() && jwt.is_some() {
			let mut validation = Validation::default();
			validation.set_audience(&[discord_app_id.unwrap().to_string()]);

			match decode::<Claims>(
				&jwt.unwrap().replace("Bearer ", ""),
				&DecodingKey::from_secret(&AUTH_CONFIG.secret_token.as_ref()),
				&validation
			) {
				Ok(_token_claims) => Outcome::Success(Auth {}),
				Err(_err) => {
					info!("{:?}", _err);
					Outcome::Forward(Status::Forbidden)
				}
			}
		} else {
			Outcome::Forward(Status::Unauthorized)
		}
	}
}

impl<'r> Responder<'r, 'r> for AuthApiError {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(self.to_string());
		let mut response = Response::build_from(json.respond_to(&request).unwrap());

		response
			.raw_header("x-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON)
			.status(Status::InternalServerError)
			.ok()
	}
}

impl Display for AuthApiError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AuthApiError::AuthError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}

impl From<JsonWebTokenError> for AuthApiError {
	fn from(_value: JsonWebTokenError) -> Self { Self::AuthError }
}

impl Error for AuthApiError {}
