use rocket::serde::{Deserialize, Serialize};

use crate::domain::model::request_rating;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum RequestRating {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten
}

impl Into<request_rating::RequestRating> for RequestRating {
	fn into(self) -> request_rating::RequestRating {
		match self {
			RequestRating::One => request_rating::RequestRating::One,
			RequestRating::Two => request_rating::RequestRating::Two,
			RequestRating::Three => request_rating::RequestRating::Three,
			RequestRating::Four => request_rating::RequestRating::Four,
			RequestRating::Five => request_rating::RequestRating::Five,
			RequestRating::Six => request_rating::RequestRating::Six,
			RequestRating::Seven => request_rating::RequestRating::Seven,
			RequestRating::Eight => request_rating::RequestRating::Eight,
			RequestRating::Nine => request_rating::RequestRating::Nine,
			RequestRating::Ten => request_rating::RequestRating::Ten
		}
	}
}
