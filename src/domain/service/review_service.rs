use crate::domain::model::error::level_review_error::LevelReviewError;
use crate::domain::model::review::Review;

pub trait ReviewService {
    async fn review_level(self,
                          level_id: u64,
                          reviewer_discord_id: u64,
                          discord_message_id: u64,
                          review_contents: String) -> Result<Review, LevelReviewError>;
}