use rocket_framework::http::Status;
use rocket_framework::Request;
use rocket_framework::request::{FromRequest, Outcome};
use crate::domain::model::error::level_request_error::LevelRequestError;

pub struct DiscordID(u64);

impl From<DiscordID> for u64 {
    fn from(value: DiscordID) -> Self { value.0.into() }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DiscordID {
    type Error = LevelRequestError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(discord_id) = request.headers().get_one("X-Discord-ID") {
            Outcome::Success(DiscordID(discord_id.parse::<u64>().unwrap()))
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}