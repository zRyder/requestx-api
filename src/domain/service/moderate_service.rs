use crate::domain::model::{
	error::moderator_error::ModeratorError,
	gd_level::GDLevelRequest,
	moderator::{SuggestedRating, SuggestedScore}
};

pub trait ModerateService {
	async fn send_level(
		&self,
		level_id: u64,
		suggested_rating: SuggestedRating,
		suggested_score: SuggestedScore
	) -> Result<GDLevelRequest, ModeratorError>;
}
