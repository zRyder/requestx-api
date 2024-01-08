use sea_orm::{DatabaseConnection, DbConn, DbErr, DeleteResult, EntityTrait, InsertResult};

use crate::adapter::mysql::{
	model::{prelude, prelude::Review, review},
	review_repository::ReviewRepository
};

pub struct MySqlReviewRepository<'a> {
	db_conn: &'a DatabaseConnection
}

impl<'a> ReviewRepository for MySqlReviewRepository<'a> {
	async fn create_record(
		self,
		record: review::ActiveModel
	) -> Result<InsertResult<review::ActiveModel>, DbErr> {
		Review::insert(record).exec(self.db_conn).await
	}

	async fn get_record(
		&self,
		level_id: u64,
		discord_id: u64
	) -> Result<Option<review::Model>, DbErr> {
		Review::find_by_id((level_id, discord_id))
			.one(self.db_conn)
			.await
	}

	async fn update_record(self, record: review::ActiveModel) -> Result<review::Model, DbErr> {
		Review::update(record).exec(self.db_conn).await
	}

	async fn delete_record(self, record: review::ActiveModel) -> Result<DeleteResult, DbErr> {
		Review::delete(record).exec(self.db_conn).await
	}
}

impl<'a> MySqlReviewRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self { MySqlReviewRepository { db_conn } }
}
