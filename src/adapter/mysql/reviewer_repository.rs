use sea_orm::{
	ColumnTrait, DatabaseConnection, DbConn, DbErr, EntityTrait, InsertResult, QueryFilter,
};

use crate::adapter::mysql::model::{
	prelude::Reviewer,
	reviewer,
	reviewer::{ActiveModel, Model},
};

pub struct ReviewerRepository<'a> {
	db_conn: &'a DatabaseConnection,
}

// TODO: Figure out testing with lifetime param
// #[cfg_attr(test, mockall::automock)]
impl<'a> ReviewerRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self {
		ReviewerRepository { db_conn }
	}

	pub async fn create_record(
		&self,
		record: ActiveModel,
	) -> Result<InsertResult<ActiveModel>, DbErr> {
		Reviewer::insert(record).exec(self.db_conn).await
	}

	pub async fn get_record(
		&self,
		reviewer_discord_id: u64,
		is_active: Option<bool>,
	) -> Result<Option<Model>, DbErr> {
		if let Some(active_toggle) = is_active {
			Reviewer::find_by_id(reviewer_discord_id)
				.filter(reviewer::Column::Active.eq(active_toggle))
				.one(self.db_conn)
				.await
		} else {
			Reviewer::find_by_id(reviewer_discord_id)
				.one(self.db_conn)
				.await
		}
	}

	pub async fn update_record(&self, record: ActiveModel) -> Result<Model, DbErr> {
		Reviewer::update(record).exec(self.db_conn).await
	}
}
