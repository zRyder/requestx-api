use sea_orm::{DatabaseConnection, DbErr, DeleteResult, EntityTrait, InsertResult};
use crate::adapter::mysql::model::prelude::Moderator;
use crate::adapter::mysql::model::moderator::{ActiveModel, Model};
use crate::adapter::mysql::moderator_repository::ModeratorRepository;

pub struct MySqlModeratorRepository<'a> {
    db_conn: &'a DatabaseConnection
}

impl<'a> ModeratorRepository for MySqlModeratorRepository<'a> {
    async fn create_record(&self, record: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
        Moderator::insert(record).exec(self.db_conn).await
    }

    async fn get_record(&self, level_id: u64) -> Result<Option<Model>, DbErr> {
        Moderator::find_by_id(level_id).one(self.db_conn).await
    }

    async fn update_record(&self, record: ActiveModel) -> Result<Model, DbErr> {
        Moderator::update(record).exec(self.db_conn).await
    }

    async fn delete_record(&self, record: ActiveModel) -> Result<DeleteResult, DbErr> {
        Moderator::delete(record).exec(self.db_conn).await
    }
}