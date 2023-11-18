use mockall::automock;
use sea_orm::{DbErr, DeleteResult, InsertResult};

use crate::adapter::mysql::model::{level_request, level_request::ActiveModel};

#[automock]
#[async_trait]
pub trait LevelRequestRepository {
	async fn create_record(self, record: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr>;

	async fn get_record(&self, level_id: u64) -> Result<Option<level_request::Model>, DbErr>;

	async fn update_record(self, record: ActiveModel) -> Result<level_request::Model, DbErr>;

	async fn delete_record(self, record: ActiveModel) -> Result<DeleteResult, DbErr>;
}
