use dash_rs::request::moderator::{SuggestedFeatureScore, SuggestedStars};
use sea_orm::ActiveValue;

use crate::adapter::mysql::model::{
	moderator, moderator::Model, sea_orm_active_enums, sea_orm_active_enums::Score
};

#[derive(Clone, Copy, Debug)]
pub struct Moderator {
	pub level_id: u64,
	pub suggested_score: SuggestedScore,
	pub suggested_rating: SuggestedRating
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuggestedRating {
	Rate,
	Feature,
	Epic,
	Legendary,
	Mythic
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuggestedScore {
	NoRate,
	Rated,
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

impl Into<moderator::ActiveModel> for Moderator {
	fn into(self) -> moderator::ActiveModel {
		moderator::ActiveModel {
			level_id: ActiveValue::Set(self.level_id),
			score: ActiveValue::Set(self.suggested_score.into()),
			rating: ActiveValue::Set(self.suggested_rating.into())
		}
	}
}

impl From<moderator::Model> for Moderator {
	fn from(value: Model) -> Self {
		Self {
			level_id: value.level_id,
			suggested_score: SuggestedScore::from(value.score),
			suggested_rating: SuggestedRating::from(value.rating)
		}
	}
}

impl From<Score> for SuggestedScore {
	fn from(value: Score) -> Self {
		match value {
			Score::NoRate => Self::NoRate,
			Score::Rated => Self::Rated,
			Score::One => Self::One,
			Score::Two => Self::Two,
			Score::Three => Self::Three,
			Score::Four => Self::Four,
			Score::Five => Self::Five,
			Score::Six => Self::Six,
			Score::Seven => Self::Seven,
			Score::Eight => Self::Eight,
			Score::Nine => Self::Nine,
			Score::Ten => Self::Ten
		}
	}
}

impl Into<Score> for SuggestedScore {
	fn into(self) -> Score {
		match self {
			SuggestedScore::NoRate => Score::NoRate,
			SuggestedScore::Rated => Score::Rated,
			SuggestedScore::One => Score::One,
			SuggestedScore::Two => Score::Two,
			SuggestedScore::Three => Score::Three,
			SuggestedScore::Four => Score::Four,
			SuggestedScore::Five => Score::Five,
			SuggestedScore::Six => Score::Six,
			SuggestedScore::Seven => Score::Seven,
			SuggestedScore::Eight => Score::Eight,
			SuggestedScore::Nine => Score::Nine,
			SuggestedScore::Ten => Score::Ten
		}
	}
}

impl Into<SuggestedStars> for SuggestedScore {
	fn into(self) -> SuggestedStars {
		match self {
			SuggestedScore::One => SuggestedStars::One,
			SuggestedScore::Two => SuggestedStars::Two,
			SuggestedScore::Three => SuggestedStars::Three,
			SuggestedScore::Four => SuggestedStars::Four,
			SuggestedScore::Five => SuggestedStars::Five,
			SuggestedScore::Six => SuggestedStars::Six,
			SuggestedScore::Seven => SuggestedStars::Seven,
			SuggestedScore::Eight => SuggestedStars::Eight,
			SuggestedScore::Nine => SuggestedStars::Nine,
			SuggestedScore::Ten => SuggestedStars::Ten,
			_ => unreachable!()
		}
	}
}

impl From<sea_orm_active_enums::Rating> for SuggestedRating {
	fn from(value: sea_orm_active_enums::Rating) -> Self {
		match value {
			sea_orm_active_enums::Rating::Rate => Self::Rate,
			sea_orm_active_enums::Rating::Feature => Self::Feature,
			sea_orm_active_enums::Rating::Epic => Self::Epic,
			sea_orm_active_enums::Rating::Legendary => Self::Legendary,
			sea_orm_active_enums::Rating::Mythic => Self::Mythic
		}
	}
}

impl Into<sea_orm_active_enums::Rating> for SuggestedRating {
	fn into(self) -> sea_orm_active_enums::Rating {
		match self {
			SuggestedRating::Rate => sea_orm_active_enums::Rating::Rate,
			SuggestedRating::Feature => sea_orm_active_enums::Rating::Feature,
			SuggestedRating::Epic => sea_orm_active_enums::Rating::Epic,
			SuggestedRating::Legendary => sea_orm_active_enums::Rating::Legendary,
			SuggestedRating::Mythic => sea_orm_active_enums::Rating::Mythic
		}
	}
}

impl Into<SuggestedFeatureScore> for SuggestedRating {
	fn into(self) -> SuggestedFeatureScore {
		match self {
			SuggestedRating::Rate => SuggestedFeatureScore::Rate,
			SuggestedRating::Feature => SuggestedFeatureScore::Featured,
			SuggestedRating::Epic => SuggestedFeatureScore::Epic,
			SuggestedRating::Legendary => SuggestedFeatureScore::Legendary,
			SuggestedRating::Mythic => SuggestedFeatureScore::Mythic
		}
	}
}
