use sea_orm::{DbErr, DeleteResult, InsertResult};

use crate::adapter::mysql::model::reviewer;

#[cfg_attr(test, mockall::automock)]
pub trait ReviewerRepository {
	async fn create_record(
		&self,
		record: reviewer::ActiveModel
	) -> Result<InsertResult<reviewer::ActiveModel>, DbErr>;

	async fn get_record(
		&self,
		reviewer_discord_id: u64,
		is_active: Option<bool>
	) -> Result<Option<reviewer::Model>, DbErr>;

	async fn get_record_ignore_active(
		&self,
		reviewer_discord_id: u64
	) -> Result<Option<reviewer::Model>, DbErr>;

	async fn update_record(&self, record: reviewer::ActiveModel) -> Result<reviewer::Model, DbErr>;

	async fn delete_record(&self, record: reviewer::ActiveModel) -> Result<DeleteResult, DbErr>;
}
