use sea_orm::DbErr;
use crate::adapter::mysql::level_request_repository::LevelRequestRepository;
use crate::adapter::mysql::model::review;
use crate::adapter::mysql::review_repository::ReviewRepository;
use crate::domain::model::error::level_review_error::LevelReviewError;
use crate::domain::model::review::Review;
use crate::domain::service::review_service::ReviewService;

pub struct LevelReviewService<R: ReviewRepository, L: LevelRequestRepository> {
    review_repository: R,
    level_request_repository: L
}

impl<R: ReviewRepository, L: LevelRequestRepository> ReviewService for LevelReviewService<R, L> {
    async fn review_level(
        self,
        level_id: u64,
        reviewer_discord_id: u64,
        discord_message_id: u64,
        review_contents: String
    ) -> Result<Review, LevelReviewError> {
        match self.level_request_repository.get_record(level_id).await {
            Ok(potential_level_request) => {
                let mut level_review = Review {
                    reviewer_discord_id,
                    discord_message_id,
                    level_id,
                    review_contents,
                    is_update: false,
                };
                let level_review_storable: review::ActiveModel = level_review.clone().into();
                if let Some(level_request) = potential_level_request {
                    match self.review_repository.get_record(level_review.level_id, level_review.reviewer_discord_id).await {
                        Ok(potential_level_review) => {
                            if let Some(existing_level_review) = potential_level_review {
                                info!("Updating existing level review for level: {:?}", level_request);
                                if let Err(update_error) = self.review_repository.update_record(level_review_storable).await {
                                    error!("Error updating level review from database: {}", update_error);
                                    Err(LevelReviewError::DatabaseError(update_error))
                                } else {
                                    level_review.is_update = true;
                                    Ok(level_review)
                                }
                            } else {
                                if let Err(insertion_error) = self.review_repository.create_record(level_review_storable).await {
                                    error!("Error inserting level review from database: {}", insertion_error);
                                    Err(LevelReviewError::DatabaseError(insertion_error))
                                } else {
                                    Ok(level_review)
                                }
                            }
                        }
                        Err(error) => {
                            error!("Error reading level review from database: {}", error);
                            Err(LevelReviewError::DatabaseError(error))
                        }
                    }
                }
                else {
                    warn!("Reviewer {} attempted to write review for level ID {} which does not exist", level_review.reviewer_discord_id, level_review.level_id);
                    Err(LevelReviewError::LevelRequestDoesNotExist)
                }
            }
            Err(error) => {
                error!("Error reading level request from database: {}", error);
                Err(LevelReviewError::DatabaseError(error))
            }
        }
    }
}

impl<R: ReviewRepository, L: LevelRequestRepository> LevelReviewService<R, L> {
    pub fn new(review_repository: R, level_request_repository: L) -> Self {
        LevelReviewService {
            review_repository,
            level_request_repository
        }
    }
}