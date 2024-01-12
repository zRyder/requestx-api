use sea_orm::{DbErr, DeleteResult, InsertResult};
use crate::adapter::mysql::model::moderator;

#[cfg_attr(test, mockall::automock)]
pub trait ModeratorRepository {
    async fn create_record(
        &self,
        record: moderator::ActiveModel
    ) -> Result<InsertResult<moderator::ActiveModel>, DbErr>;

    async fn get_record(
        &self,
        level_id: u64
    ) -> Result<Option<moderator::Model>, DbErr>;

    async fn update_record(
        &self,
        record: moderator::ActiveModel
    ) -> Result<moderator::Model, DbErr>;

    async fn delete_record(
        &self,
        record: moderator::ActiveModel
    ) -> Result<DeleteResult, DbErr>;
}