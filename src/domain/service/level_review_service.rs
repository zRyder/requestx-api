use crate::{
	adapter::mysql::review_repository::ReviewRepository,
	domain::{
		model::{error::level_review_error::LevelReviewError, review::LevelReview},
		service::level_request_service::LevelRequestService
	},
	rocket::common::config::common_config::APP_CONFIG
};

pub struct LevelReviewService<'a> {
	review_repository: &'a ReviewRepository<'a>,
	level_request_service: &'a LevelRequestService<'a>
}

impl<'a> LevelReviewService<'a> {
	pub fn new(
		review_repository: &'a ReviewRepository,
		level_request_service: &'a LevelRequestService
	) -> Self {
		LevelReviewService {
			review_repository,
			level_request_service
		}
	}

	pub async fn get_level_review(
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

	pub async fn review_level(
		&self,
		level_id: u64,
		reviewer_discord_id: u64,
		discord_message_id: u64,
		review_contents: String
	) -> Result<LevelReview, LevelReviewError> {
		let level_request_result = if reviewer_discord_id.eq(&APP_CONFIG
			.get()
			.unwrap()
			.server_config
			.discord_bot_admin_id)
		{
			self.level_request_service
				.get_level_request(level_id, None)
				.await
		} else {
			self.level_request_service
				.get_level_request(level_id, Some(true))
				.await
		};

		let level_request_to_review =
			level_request_result
				.map(Ok)
				.unwrap_or_else(|get_level_request_error| {
					error!(
						"Level request does not exist or feedback was not requested {}",
						get_level_request_error
					);
					Err(LevelReviewError::LevelRequestDoesNotExist)
				})?;

		let mut level_review = LevelReview {
			reviewer_discord_id,
			discord_message_id,
			level_id,
			review_contents,
			is_update: false
		};

		let existing_level_review = self
			.review_repository
			.get_record(level_id, reviewer_discord_id)
			.await
			.map_err(|query_level_review_error| {
				error!(
					"Error querying level review from database: {}",
					query_level_review_error
				);
				LevelReviewError::DatabaseError(query_level_review_error)
			})?
			.map(LevelReview::from);

		match existing_level_review {
			Some(existing_level_review) => {
				info!(
					"Updating existing level review for level: {:?}",
					level_request_to_review
				);
				level_review.discord_message_id = existing_level_review.discord_message_id;

				if let Err(update_error) = self
					.review_repository
					.update_record(level_review.clone().into())
					.await
				{
					error!(
						"Error updating level review from database: {}",
						update_error
					);
					return Err(LevelReviewError::DatabaseError(update_error));
				} else {
					level_review.is_update = true;
				}
			}
			None => {
				if let Err(create_level_review_error) = self
					.review_repository
					.create_record(level_review.clone().into())
					.await
				{
					error!(
						"Error inserting level review from database: {}",
						create_level_review_error
					);
					return Err(LevelReviewError::DatabaseError(create_level_review_error));
				}
			}
		}

		Ok(level_review)
	}

	pub async fn update_level_request_thread_id(
		&self,
		level_id: u64,
		discord_id: u64,
		discord_message_id: u64
	) -> Result<(), LevelReviewError> {
		let mut existing_level_review = self
			.review_repository
			.get_record(level_id, discord_id)
			.await
			.map_err(|query_level_review_error| {
				error!(
					"Error querying level review from database: {}",
					query_level_review_error
				);
				LevelReviewError::DatabaseError(query_level_review_error)
			})?
			.map(LevelReview::from)
			.map(Ok)
			.unwrap_or_else(|| {
				error!(
					"Level review for level request ID {} by {} does not exist",
					level_id, discord_id
				);
				Err(LevelReviewError::LevelRequestDoesNotExist)
			})?;

		existing_level_review.discord_message_id = discord_message_id;
		if let Err(update_level_review_error) = self
			.review_repository
			.update_record(existing_level_review.into())
			.await
		{
			error!(
				"Error updating level review by {} with level ID: {}: {}",
				discord_id, level_id, update_level_review_error
			);
			return Err(LevelReviewError::DatabaseError(update_level_review_error));
		}

		Ok(())
	}
}
