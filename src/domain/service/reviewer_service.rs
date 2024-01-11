use crate::domain::model::{error::reviewer_error::ReviewerError, reviewer::Reviewer};

pub trait ReviewerService {
	async fn get_reviewer(
		self,
		reviewer_discord_id: u64,
		include_active: bool
	) -> Result<Reviewer, ReviewerError>;

	async fn create_reviewer(self, reviewer_discord_id: u64) -> Result<(), ReviewerError>;

	async fn remove_reviewer(self, reviewer_discord_id: u64) -> Result<(), ReviewerError>;
}
