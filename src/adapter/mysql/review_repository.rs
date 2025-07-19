use sea_orm::{DbErr, DeleteResult, InsertResult};

use crate::adapter::mysql::model::review;

#[cfg_attr(test, mockall::automock)]
pub trait ReviewRepository {
	async fn create_record(
		&self,
		record: review::ActiveModel
	) -> Result<InsertResult<review::ActiveModel>, DbErr>;

	async fn get_record(
		&self,
		level_id: u64,
		discord_id: u64
	) -> Result<Option<review::Model>, DbErr>;

	async fn update_record(&self, record: review::ActiveModel) -> Result<review::Model, DbErr>;

	async fn delete_record(&self, record: review::ActiveModel) -> Result<DeleteResult, DbErr>;
}
