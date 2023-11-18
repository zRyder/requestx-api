use rocket::serde::Deserialize;

use crate::domain::model::api::request_rating;

#[derive(Deserialize)]
pub struct LevelRequestApiRequest {
	level_id: u64,
	youtube_vidoe_link: String,
	discord_id: String,
	request_rating: request_rating::RequestRating
}
