use std::error::Error;
use std::fmt::{Display, Formatter};
use chrono::Local;
use jsonwebtoken::errors::Error as JsonWebTokenError;
use rocket_framework::http::{ContentType, Status};
use rocket_framework::{Request, Response};
use rocket_framework::request::{FromRequest, Outcome};
use rocket_framework::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket_framework::serde::json::Json;
use crate::domain::model::error::level_request_error::LevelRequestError;
use crate::rocket::common::config::auth_config::AUTH_CONFIG;

#[derive(Deserialize)]
pub struct AuthApiRequest {
    pub discord_app_id: u64,
    _access_token: String
}

#[derive(Serialize)]
pub struct AuthApiResponse {
    jwt: String
}

#[derive(Debug, PartialEq)]
pub enum AuthApiError {
    AuthError
}

impl AuthApiResponse {
    pub fn new(jwt: String) -> Self {
        Self {
            jwt
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthApiRequest {
    type Error = LevelRequestError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let discord_app_id = request.headers().get_one("X-REQUESTX-DISCORD-APP-ID");
        let access_token = request.headers().get_one("X-REQUESTX-ACCESS-TOKEN");
        if discord_app_id.is_some() && access_token.is_some() {
            if discord_app_id.unwrap().ne(&AUTH_CONFIG.discord_app_id) {
                Outcome::Forward(Status::Unauthorized)
            } else if access_token.unwrap().ne(&AUTH_CONFIG.access_token) {
                Outcome::Forward(Status::Forbidden)
            }
            else {
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

impl<'r > Responder<'r, 'r> for AuthApiError {
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
    fn from(_value: JsonWebTokenError) -> Self {
        Self::AuthError
    }
}

impl Error for AuthApiError {}

