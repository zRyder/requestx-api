use sea_orm::ActiveValue::Set;

use crate::{
	adapter::mysql::{model::review::ActiveModel, review_repository::ReviewRepository},
	domain::{
		model::{
			error::{level_request_error::LevelRequestError, level_review_error::LevelReviewError},
			review::LevelReview
		},
		service::{request_service::RequestService, review_service::ReviewService}
	},
	rocket::common::config::client_config::CLIENT_CONFIG
};

pub struct LevelReviewService<'a, R: ReviewRepository, L: RequestService> {
	review_repository: &'a R,
	level_request_service: &'a L
}

impl<'a, R: ReviewRepository, L: RequestService> ReviewService for LevelReviewService<'a, R, L> {
	async fn get_level_review(
		&self,
		level_id: u64,
		discord_id: u64
	) -> Result<LevelReview, LevelReviewError> {
		match self
			.review_repository
			.get_record(level_id, discord_id)
			.await
		{
			Ok(Some(level_review)) => Ok(LevelReview::from(level_review)),
			Ok(None) => {
				warn!(
					"Level review for ID {} by {} does not exist",
					level_id, discord_id
				);
				Err(LevelReviewError::LevelRequestDoesNotExist)
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
		&self,
		level_id: u64,
		reviewer_discord_id: u64,
		discord_message_id: u64,
		review_contents: String
	) -> Result<LevelReview, LevelReviewError> {
		let level_request_result = if reviewer_discord_id.eq(&CLIENT_CONFIG.discord_bot_admin_id) {
			self.level_request_service
				.get_level_request(level_id, None)
				.await
		} else {
			self.level_request_service
				.get_level_request(level_id, Some(true))
				.await
		};

		match level_request_result {
			Ok(level_request) => {
				let mut level_review = LevelReview {
					reviewer_discord_id,
					discord_message_id,
					level_id,
					review_contents,
					is_update: false
				};
				let mut level_review_storable: ActiveModel = level_review.clone().into();

				match self
					.review_repository
					.get_record(level_id, reviewer_discord_id)
					.await
				{
					Ok(Some(existing_level_review)) => {
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
					}
					Ok(None) => {
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
							Ok(level_review)
						}
					}
					Err(error) => {
						error!("Error reading level review from database: {}", error);
						Err(LevelReviewError::DatabaseError(error))
					}
				}
			}
			Err(LevelRequestError::LevelRequestDoesNotExist) => {
				warn!(
					"Reviewer {} attempted to write review for level ID \
						{} which does not exist or feedback was not requested",
					reviewer_discord_id, level_id
				);
				Err(LevelReviewError::LevelRequestDoesNotExist)
			}
			Err(LevelRequestError::DatabaseError(db_err)) => {
				Err(LevelReviewError::DatabaseError(db_err))
			}
			Err(_) => {
				unreachable!()
			}
		}
	}

	async fn update_level_request_thread_id(
		&self,
		level_id: u64,
		discord_id: u64,
		discord_message_id: u64
	) -> Result<(), LevelReviewError> {
		match self.get_level_review(level_id, discord_id).await {
			Ok(level_review) => {
				let mut update_level_review_storable: ActiveModel = level_review.into();
				update_level_review_storable.message_id = Set(discord_message_id);

				if let Err(db_err) = self
					.review_repository
					.update_record(update_level_review_storable)
					.await
				{
					error!(
						"Error updating level review by {} with level ID: {}: {}",
						discord_id, level_id, db_err
					);
					Err(LevelReviewError::DatabaseError(db_err))
				} else {
					Ok(())
				}
			}
			Err(LevelReviewError::LevelRequestDoesNotExist) => {
				warn!("Level request with ID: {} does not exist", level_id);
				Err(LevelReviewError::LevelRequestDoesNotExist)
			}
			Err(error) => Err(error)
		}
	}
}

impl<'a, R: ReviewRepository, L: RequestService> LevelReviewService<'a, R, L> {
	pub fn new(review_repository: &'a R, level_request_service: &'a L) -> Self {
		LevelReviewService {
			review_repository,
			level_request_service
		}
	}
}
