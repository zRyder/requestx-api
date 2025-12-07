use sea_orm::{DatabaseConnection, DbConn, DbErr, EntityTrait, InsertResult};

use crate::adapter::mysql::model::{prelude::Review, review};

pub struct ReviewRepository<'a> {
	db_conn: &'a DatabaseConnection,
}

// TODO: Figure out testing with lifetime param
// #[cfg_attr(test, mockall::automock)]
impl<'a> ReviewRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self {
		ReviewRepository { db_conn }
	}

	pub async fn create_record(
		&self,
		record: review::ActiveModel,
	) -> Result<InsertResult<review::ActiveModel>, DbErr> {
		Review::insert(record).exec(self.db_conn).await
	}

	pub async fn get_record(
		&self,
		level_id: u64,
		discord_id: u64,
	) -> Result<Option<review::Model>, DbErr> {
		Review::find_by_id((level_id, discord_id))
			.one(self.db_conn)
			.await
	}

	pub async fn update_record(&self, record: review::ActiveModel) -> Result<review::Model, DbErr> {
		Review::update(record).exec(self.db_conn).await
	}
}
