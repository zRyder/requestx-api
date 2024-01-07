use sea_orm::{
    DatabaseConnection, DbConn, DbErr, DeleteResult, EntityTrait, InsertResult
};

use crate::adapter::mysql::{
    model::{user::ActiveModel, prelude::*, *}
};
use crate::adapter::mysql::user_repository::UserRepository;

pub struct MySqlUserRepository<'a> {
    db_conn: &'a DatabaseConnection
}

impl<'a> UserRepository for MySqlUserRepository<'a> {
    async fn create_record(self, record: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
        User::insert(record).exec(self.db_conn).await
    }

    async fn get_record(&self, discord_id: u64) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(discord_id).one(self.db_conn).await
    }

    async fn update_record(self, record: ActiveModel) -> Result<user::Model, DbErr> {
        User::update(record).exec(self.db_conn).await
    }

    async fn delete_record(self, record: ActiveModel) -> Result<DeleteResult, DbErr> {
        User::delete(record).exec(self.db_conn).await
    }
}

impl<'a> MySqlUserRepository<'a> {
    pub fn new(db_conn: &'a DbConn) -> Self { MySqlUserRepository { db_conn } }
}