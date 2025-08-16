use sea_orm::{DatabaseConnection, DbConn, DbErr, EntityTrait, InsertResult};

use crate::adapter::mysql::model::{prelude::*, user, user::ActiveModel};

pub struct UserRepository<'a> {
	db_conn: &'a DatabaseConnection
}

// TODO: Figure out testing with lifetime param
// #[cfg_attr(test, mockall::automock)]
impl<'a> UserRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self { UserRepository { db_conn } }

	pub async fn create_record(
		&self,
		record: ActiveModel
	) -> Result<InsertResult<ActiveModel>, DbErr> {
		User::insert(record).exec(self.db_conn).await
	}

	pub async fn get_record(&self, discord_id: u64) -> Result<Option<user::Model>, DbErr> {
		User::find_by_id(discord_id).one(self.db_conn).await
	}

	pub async fn update_record(&self, record: ActiveModel) -> Result<user::Model, DbErr> {
		User::update(record).exec(self.db_conn).await
	}
}
