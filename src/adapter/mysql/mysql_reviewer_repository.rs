use sea_orm::{
	ColumnTrait, DatabaseConnection, DbConn, DbErr, DeleteResult, EntityTrait, InsertResult,
	QueryFilter
};

use crate::adapter::mysql::{
	model::{
		prelude::Reviewer,
		reviewer,
		reviewer::{ActiveModel, Model}
	},
	reviewer_repository::ReviewerRepository
};

pub struct MySqlReviewerRepository<'a> {
	db_conn: &'a DatabaseConnection
}

impl<'a> ReviewerRepository for MySqlReviewerRepository<'a> {
	async fn create_record(self, record: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
		Reviewer::insert(record).exec(self.db_conn).await
	}

	async fn get_record(
		&self,
		reviewer_discord_id: u64,
		is_active: bool
	) -> Result<Option<Model>, DbErr> {
		Reviewer::find_by_id(reviewer_discord_id)
			.filter(reviewer::Column::Active.eq(is_active))
			.one(self.db_conn)
			.await
	}

	async fn get_record_ignore_active(
		&self,
		reviewer_discord_id: u64
	) -> Result<Option<Model>, DbErr> {
		Reviewer::find_by_id(reviewer_discord_id)
			.one(self.db_conn)
			.await
	}

	async fn update_record(self, record: ActiveModel) -> Result<Model, DbErr> {
		Reviewer::update(record).exec(self.db_conn).await
	}

	async fn delete_record(self, record: ActiveModel) -> Result<DeleteResult, DbErr> {
		Reviewer::delete(record).exec(self.db_conn).await
	}
}

impl<'a> MySqlReviewerRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self { MySqlReviewerRepository { db_conn } }
}
