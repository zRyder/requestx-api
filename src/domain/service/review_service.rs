use crate::domain::model::{error::level_review_error::LevelReviewError, review::LevelReview};

pub trait ReviewService {
	async fn get_level_review(
		self,
		level_id: u64,
		discord_id: u64
	) -> Result<LevelReview, LevelReviewError>;

	async fn review_level(
		self,
		level_id: u64,
		reviewer_discord_id: u64,
		discord_message_id: u64,
		review_contents: String
	) -> Result<LevelReview, LevelReviewError>;

	async fn update_level_request_thread_id(
		self,
		level_id: u64,
		discord_id: u64,
		discord_message_id: u64
	) -> Result<(), LevelReviewError>;
}
