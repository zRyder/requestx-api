use sea_orm::{DatabaseConnection, DbConn, DbErr, EntityTrait, InsertResult};

use crate::adapter::mysql::model::{
	moderator::{ActiveModel, Model},
	prelude::Moderator,
};

pub struct ModeratorRepository<'a> {
	db_conn: &'a DatabaseConnection,
}

// TODO: Figure out testing with lifetime param
// #[cfg_attr(test, mockall::automock)]
impl<'a> ModeratorRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self {
		ModeratorRepository { db_conn }
	}

	pub async fn create_record(
		&self,
		record: ActiveModel,
	) -> Result<InsertResult<ActiveModel>, DbErr> {
		Moderator::insert(record).exec(self.db_conn).await
	}

	pub async fn get_record(&self, level_id: u64) -> Result<Option<Model>, DbErr> {
		Moderator::find_by_id(level_id).one(self.db_conn).await
	}

	pub async fn update_record(&self, record: ActiveModel) -> Result<Model, DbErr> {
		Moderator::update(record).exec(self.db_conn).await
	}
}
