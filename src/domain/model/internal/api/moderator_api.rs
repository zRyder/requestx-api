use std::fmt::{Display, Formatter};

use chrono::Local;
use rocket_framework::{
	http::{ContentType, Status},
	response::Responder,
	serde::json::Json,
	Request, Response
};
use serde_derive::{Deserialize, Serialize};

use crate::domain::model::moderator;

#[derive(Deserialize)]
pub struct PostModeratorApiRequest {
	pub level_id: u64,
	pub suggested_score: SuggestedScore,
	pub suggested_rating: SuggestedRating
}

pub enum ModeratorApiResponseError {
	LevelRequestDoesNotExist,
	UnsendableLevel,
	ModeratorError
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum SuggestedScore {
	NoRate,
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

impl<'r> Responder<'r, 'r> for ModeratorApiResponseError {
	fn respond_to(self, request: &'r Request<'_>) -> rocket_framework::response::Result<'r> {
		let json = Json(self.to_string());
		let mut response = Response::build_from(json.respond_to(&request).unwrap());
		response
			.raw_header("x-requestx-timestamp", format!("{}", Local::now()))
			.header(ContentType::JSON);

		match self {
			ModeratorApiResponseError::LevelRequestDoesNotExist => {
				response.status(Status::NotFound);
			}
			ModeratorApiResponseError::UnsendableLevel => {
				response.status(Status::BadRequest);
			}
			ModeratorApiResponseError::ModeratorError => {
				response.status(Status::InternalServerError);
			}
		};

		response.ok()
	}
}

impl Display for ModeratorApiResponseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ModeratorApiResponseError::LevelRequestDoesNotExist => {
				write!(f, "{{\"message\": \"Level request does not exist\"}}")
			}
			ModeratorApiResponseError::UnsendableLevel => {
				write!(f, "{{\"message\": \"The send level is already rated, deleted, or attempted to unsend a level\"}}")
			}
			ModeratorApiResponseError::ModeratorError => {
				write!(f, "{{\"message\": \"Internal server error\"}}")
			}
		}
	}
}

impl Into<moderator::SuggestedScore> for SuggestedScore {
	fn into(self) -> moderator::SuggestedScore {
		match self {
			SuggestedScore::NoRate => moderator::SuggestedScore::NoRate,
			SuggestedScore::One => moderator::SuggestedScore::One,
			SuggestedScore::Two => moderator::SuggestedScore::Two,
			SuggestedScore::Three => moderator::SuggestedScore::Three,
			SuggestedScore::Four => moderator::SuggestedScore::Four,
			SuggestedScore::Five => moderator::SuggestedScore::Five,
			SuggestedScore::Six => moderator::SuggestedScore::Six,
			SuggestedScore::Seven => moderator::SuggestedScore::Seven,
			SuggestedScore::Eight => moderator::SuggestedScore::Eight,
			SuggestedScore::Nine => moderator::SuggestedScore::Nine,
			SuggestedScore::Ten => moderator::SuggestedScore::Ten
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum SuggestedRating {
	Rate,
	Feature,
	Epic,
	Legendary,
	Mythic
}

impl Into<moderator::SuggestedRating> for SuggestedRating {
	fn into(self) -> moderator::SuggestedRating {
		match self {
			SuggestedRating::Rate => moderator::SuggestedRating::Rate,
			SuggestedRating::Feature => moderator::SuggestedRating::Feature,
			SuggestedRating::Epic => moderator::SuggestedRating::Epic,
			SuggestedRating::Legendary => moderator::SuggestedRating::Legendary,
			SuggestedRating::Mythic => moderator::SuggestedRating::Mythic
		}
	}
}
