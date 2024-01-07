use rocket::serde::Deserialize;

use crate::domain::model::api::request_rating;

#[derive(Deserialize)]
pub struct LevelRequestApiRequest<'a> {
	pub level_id: u64,
	pub youtube_video_link: &'a str,
	pub discord_id: u64,
	pub request_rating: request_rating::RequestRating
}
