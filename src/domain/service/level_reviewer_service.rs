use sea_orm::{ActiveValue, IntoActiveModel};

use crate::{
	adapter::mysql::reviewer_repository::ReviewerRepository,
	domain::{
		model::{error::reviewer_error::ReviewerError, reviewer::Reviewer},
		service::reviewer_service::ReviewerService
	}
};

pub struct LevelReviewerService<R: ReviewerRepository> {
	reviewer_repository: R
}

impl<R: ReviewerRepository> ReviewerService for LevelReviewerService<R> {
	async fn get_reviewer(
		self,
		reviewer_discord_id: u64,
		include_active: bool
	) -> Result<Reviewer, ReviewerError> {
		match self
			.reviewer_repository
			.get_record(reviewer_discord_id, include_active)
			.await
		{
			Ok(potential_reviewer) => {
				if let Some(reviewer) = potential_reviewer {
					Ok(Reviewer::from(reviewer))
				} else {
					warn!("Reviewer with ID: {} does not exist", reviewer_discord_id);
					Err(ReviewerError::ReviewerDoesNotExist)
				}
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

	async fn create_reviewer(self, reviewer_discord_id: u64) -> Result<(), ReviewerError> {
		match self
			.reviewer_repository
			.get_record_ignore_active(reviewer_discord_id)
			.await
		{
			Ok(potential_existing_reviewer) => {
				if let Some(existing_reviewer) = potential_existing_reviewer {
					warn!(
						"reviewer {} already exists, updating state",
						reviewer_discord_id
					);
					let mut create_reviewer_request = existing_reviewer.into_active_model();
					create_reviewer_request.active = ActiveValue::Set(1);
					if let Err(remove_reviewer_error) = self
						.reviewer_repository
						.update_record(create_reviewer_request)
						.await
					{
						error!(
							"Error removing reviewer {}: {}",
							reviewer_discord_id, remove_reviewer_error
						);
						Err(ReviewerError::DatabaseError(remove_reviewer_error))
					} else {
						Ok(())
					}
				} else {
					let reviewer_request = Reviewer {
						discord_id: reviewer_discord_id,
						is_active: true
					};

					if let Err(create_reviewer_error) = self
						.reviewer_repository
						.create_record(reviewer_request.into())
						.await
					{
						error!(
							"Error creating reviewer {} record: {}",
							reviewer_discord_id, create_reviewer_error
						);
						Err(ReviewerError::DatabaseError(create_reviewer_error))
					} else {
						Ok(())
					}
				}
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

	async fn remove_reviewer(self, reviewer_discord_id: u64) -> Result<(), ReviewerError> {
		match self
			.reviewer_repository
			.get_record(reviewer_discord_id, true)
			.await
		{
			Ok(potential_existing_reviewer) => {
				if let Some(existing_reviewer) = potential_existing_reviewer {
					let mut remove_reviewer_request = existing_reviewer.into_active_model();
					remove_reviewer_request.active = ActiveValue::Set(0);
					if let Err(remove_reviewer_error) = self
						.reviewer_repository
						.update_record(remove_reviewer_request)
						.await
					{
						error!(
							"Error removing reviewer {}: {}",
							reviewer_discord_id, remove_reviewer_error
						);
						Err(ReviewerError::DatabaseError(remove_reviewer_error))
					} else {
						Ok(())
					}
				} else {
					warn!("Reviewer with ID: {} does not exist", reviewer_discord_id);
					Err(ReviewerError::ReviewerDoesNotExist)
				}
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
}

impl<R: ReviewerRepository> LevelReviewerService<R> {
	pub fn new(reviewer_repository: R) -> Self {
		LevelReviewerService {
			reviewer_repository
		}
	}
}
