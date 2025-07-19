use crate::domain::model::{
	error::moderator_error::ModeratorError,
	level_request::LevelRequest,
	moderator::{Moderator, SuggestedRating, SuggestedScore}
};

pub trait ModerateService {
	async fn send_level(
		&self,
		level_id: u64,
		suggested_rating: SuggestedRating,
		suggested_score: SuggestedScore
	) -> Result<(LevelRequest, Moderator), ModeratorError>;
}
