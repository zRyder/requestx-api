use sea_orm::{DbErr, DeleteResult, InsertResult};

use crate::adapter::mysql::model::{user, user::ActiveModel};

#[cfg_attr(test, mockall::automock)]
pub trait UserRepository {
	async fn create_record(&self, record: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr>;

	async fn get_record(&self, discord_id: u64) -> Result<Option<user::Model>, DbErr>;

	async fn update_record(&self, record: ActiveModel) -> Result<user::Model, DbErr>;

	async fn delete_record(&self, record: ActiveModel) -> Result<DeleteResult, DbErr>;
}
