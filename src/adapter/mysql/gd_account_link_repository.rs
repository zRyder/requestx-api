use sea_orm::{
	sea_query::OnConflict, ColumnTrait, DatabaseConnection, DbConn, DbErr, EntityTrait,
	InsertResult, QueryFilter,
};

use crate::adapter::mysql::model::{
	gd_account_link, gd_account_link::ActiveModel, prelude::GdAccountLink,
};

pub struct GDAccountLinkRepository<'a> {
	db_conn: &'a DatabaseConnection,
}

impl<'a> GDAccountLinkRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self {
		GDAccountLinkRepository { db_conn }
	}

	pub async fn create_or_update_record(
		&self,
		record: ActiveModel,
	) -> Result<InsertResult<ActiveModel>, DbErr> {
		GdAccountLink::insert(record)
			.on_conflict(
				OnConflict::new()
					.update_columns([
						gd_account_link::Column::DiscordId,
						gd_account_link::Column::GdPlayerId,
						gd_account_link::Column::GdAccountChallenge,
						gd_account_link::Column::GdAccountHash,
						gd_account_link::Column::GdAccountPublicKey,
						gd_account_link::Column::IsGdAccountLinked,
						gd_account_link::Column::Expiry,
					])
					.to_owned(),
			)
			.exec(self.db_conn)
			.await
	}

	pub async fn get_record(
		&self,
		discord_id: u64,
	) -> Result<Option<gd_account_link::Model>, DbErr> {
		GdAccountLink::find_by_id(discord_id)
			.one(self.db_conn)
			.await
	}

	pub async fn get_verified_record_with_gd_player_id(
		&self,
		gd_player_id: u64,
	) -> Result<Option<gd_account_link::Model>, DbErr> {
		GdAccountLink::find()
			.filter(gd_account_link::Column::GdPlayerId.eq(gd_player_id))
			.filter(gd_account_link::Column::IsGdAccountLinked.eq(true))
			.one(self.db_conn)
			.await
	}
}
