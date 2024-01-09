use sea_orm::{ActiveValue::Set, IntoActiveModel};

use crate::{
	adapter::mysql::{
		level_request_repository::LevelRequestRepository,
		model::{review, review::ActiveModel},
		review_repository::ReviewRepository
	},
	domain::{
		model::{error::level_review_error::LevelReviewError, review::LevelReview},
		service::review_service::ReviewService
	}
};

pub struct LevelReviewService<R: ReviewRepository, L: LevelRequestRepository> {
	review_repository: R,
	level_request_repository: L
}

impl<R: ReviewRepository, L: LevelRequestRepository> ReviewService for LevelReviewService<R, L> {
	async fn get_level_review(
		self,
		level_id: u64,
		discord_id: u64
	) -> Result<LevelReview, LevelReviewError> {
		match self
			.review_repository
			.get_record(level_id, discord_id)
			.await
		{
			Ok(potential_level_review) => {
				if let Some(level_review_model) = potential_level_review {
					let level_review = LevelReview::from(level_review_model);
					Ok(level_review)
				} else {
					warn!(
						"Level review for ID {} by {} does not exist",
						level_id, discord_id
					);
					Err(LevelReviewError::LevelRequestDoesNotExist)
				}
			}
			Err(db_error) => {
				error!(
					"Error making get level request for level {} record database: {}",
					level_id, db_error
				);
				Err(LevelReviewError::DatabaseError(db_error))
			}
		}
	}

	async fn review_level(
		self,
		level_id: u64,
		reviewer_discord_id: u64,
		discord_message_id: u64,
		review_contents: String
	) -> Result<LevelReview, LevelReviewError> {
		match self.level_request_repository.get_record(level_id).await {
			Ok(potential_level_request) => {
				if let Some(level_request) = potential_level_request {
					match self
						.review_repository
						.get_record(level_id, reviewer_discord_id)
						.await
					{
						Ok(potential_level_review) => {
							let mut level_review = LevelReview {
								reviewer_discord_id,
								discord_message_id,
								level_id,
								review_contents,
								is_update: false
							};
							let mut level_review_storable: review::ActiveModel =
								level_review.clone().into();

							if let Some(existing_level_review) = potential_level_review {
								info!(
									"Updating existing level review for level: {:?}",
									level_request
								);
								level_review.discord_message_id = existing_level_review.message_id;
								level_review_storable.review_content =
									Set(level_review.clone().review_contents);
								level_review_storable.message_id =
									Set(level_review.clone().discord_message_id);
								if let Err(update_error) = self
									.review_repository
									.update_record(level_review_storable)
									.await
								{
									error!(
										"Error updating level review from database: {}",
										update_error
									);
									Err(LevelReviewError::DatabaseError(update_error))
								} else {
									level_review.is_update = true;
									Ok(level_review)
								}
							} else {
								if let Err(insertion_error) = self
									.review_repository
									.create_record(level_review_storable)
									.await
								{
									error!(
										"Error inserting level review from database: {}",
										insertion_error
									);
									Err(LevelReviewError::DatabaseError(insertion_error))
								} else {
									warn!("Inserting",);
									Ok(level_review)
								}
							}
						}
						Err(error) => {
							error!("Error reading level review from database: {}", error);
							Err(LevelReviewError::DatabaseError(error))
						}
					}
				} else {
					warn!("Reviewer {} attempted to write review for level ID {} which does not exist", reviewer_discord_id, level_id);
					Err(LevelReviewError::LevelRequestDoesNotExist)
				}
			}
			Err(error) => {
				error!("Error reading level request from database: {}", error);
				Err(LevelReviewError::DatabaseError(error))
			}
		}
	}

	async fn update_level_request_thread_id(
		self,
		level_id: u64,
		discord_id: u64,
		discord_message_id: u64
	) -> Result<(), LevelReviewError> {
		match self
			.review_repository
			.get_record(level_id, discord_id)
			.await
		{
			Ok(potential_level_review) => {
				if let Some(level_review) = potential_level_review {
					let mut update_level_request_storable: ActiveModel =
						level_review.into_active_model();
					update_level_request_storable.message_id = Set(discord_message_id);
					match self
						.review_repository
						.update_record(update_level_request_storable)
						.await
					{
						Ok(_) => Ok(()),
						Err(db_error) => {
							error!(
								"Error updating level review by {} with level ID: {}: {}",
								discord_id, level_id, db_error
							);
							Err(LevelReviewError::DatabaseError(db_error))
						}
					}
				} else {
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
