use rocket::serde::Deserialize;

use crate::domain::model::request_rating;

#[derive(Deserialize, Clone, Copy)]
pub enum RequestRating {
	Easy,
	Normal,
	Hard,
	Harder,
	Insane,
	Demon
}

impl Into<request_rating::RequestRating> for RequestRating {
	fn into(self) -> request_rating::RequestRating {
		match self {
			RequestRating::Easy => request_rating::RequestRating::Easy,
			RequestRating::Normal => request_rating::RequestRating::Normal,
			RequestRating::Hard => request_rating::RequestRating::Hard,
			RequestRating::Harder => request_rating::RequestRating::Harder,
			RequestRating::Insane => request_rating::RequestRating::Insane,
			RequestRating::Demon => request_rating::RequestRating::Demon
		}
	}
}
