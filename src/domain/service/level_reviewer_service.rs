use sea_orm::ActiveValue;

use crate::{
	adapter::mysql::{model::reviewer::ActiveModel, reviewer_repository::ReviewerRepository},
	domain::{
		model::{error::reviewer_error::ReviewerError, reviewer::Reviewer},
		service::reviewer_service::ReviewerService
	}
};

pub struct LevelReviewerService<'a, R: ReviewerRepository> {
	reviewer_repository: &'a R
}

impl<'a, R: ReviewerRepository> ReviewerService for LevelReviewerService<'a, R> {
	async fn get_reviewer(
		&self,
		reviewer_discord_id: u64,
		include_active: Option<bool>
	) -> Result<Reviewer, ReviewerError> {
		match self
			.reviewer_repository
			.get_record(reviewer_discord_id, include_active)
			.await
		{
			Ok(Some(level_reviewer)) => Ok(Reviewer::from(level_reviewer)),
			Ok(None) => {
				warn!("Reviewer with ID: {} does not exist", reviewer_discord_id);
				Err(ReviewerError::ReviewerDoesNotExist)
			}
			Err(get_reviewer_error) => {
				error!(
					"Error getting reviewer with ID {}: {}",
					reviewer_discord_id, get_reviewer_error
				);
				Err(ReviewerError::DatabaseError(get_reviewer_error))
			}
		}
	}

	async fn create_reviewer(&self, reviewer_discord_id: u64) -> Result<(), ReviewerError> {
		match self.get_reviewer(reviewer_discord_id, None).await {
			Ok(level_reviewer) => {
				warn!(
					"reviewer {} already exists, updating state",
					reviewer_discord_id
				);
				let mut create_reviewer_request: ActiveModel = level_reviewer.into();
				create_reviewer_request.active = ActiveValue::Set(1);

				if let Err(db_err) = self
					.reviewer_repository
					.update_record(create_reviewer_request)
					.await
				{
					error!(
						"Error removing reviewer {}: {}",
						reviewer_discord_id, db_err
					);
					return Err(ReviewerError::DatabaseError(db_err));
				}
			}
			Err(ReviewerError::ReviewerDoesNotExist) => {
				let level_reviewer = Reviewer {
					discord_id: reviewer_discord_id,
					is_active: true
				};

				if let Err(db_err) = self
					.reviewer_repository
					.create_record(level_reviewer.into())
					.await
				{
					error!(
						"Error creating reviewer {} record: {}",
						reviewer_discord_id, db_err
					);
					return Err(ReviewerError::DatabaseError(db_err));
				}
			}
			Err(reviewer_error) => return Err(reviewer_error)
		}
		Ok(())
	}

	async fn remove_reviewer(&self, reviewer_discord_id: u64) -> Result<(), ReviewerError> {
		match self.get_reviewer(reviewer_discord_id, Some(true)).await {
			Ok(existing_level_reviewer) => {
				let mut remove_reviewer_request: ActiveModel = existing_level_reviewer.into();
				remove_reviewer_request.active = ActiveValue::Set(0);

				if let Err(db_err) = self
					.reviewer_repository
					.update_record(remove_reviewer_request)
					.await
				{
					error!(
						"Error removing reviewer {}: {}",
						reviewer_discord_id, db_err
					);
					return Err(ReviewerError::DatabaseError(db_err));
				}
			}
			Err(reviewer_error) => return Err(reviewer_error)
		}

		Ok(())
	}
}

impl<'a, R: ReviewerRepository> LevelReviewerService<'a, R> {
	pub fn new(reviewer_repository: &'a R) -> Self {
		LevelReviewerService {
			reviewer_repository
		}
	}
}
