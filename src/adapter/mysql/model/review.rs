//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "review")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub level_id: u64,
	#[sea_orm(primary_key, auto_increment = false)]
	pub discord_id: u64,
	pub message_id: u64,
	pub review_content: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::level_request::Entity",
		from = "Column::LevelId",
		to = "super::level_request::Column::LevelId",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	LevelRequest,
	#[sea_orm(
		belongs_to = "super::reviewer::Entity",
		from = "Column::DiscordId",
		to = "super::reviewer::Column::DiscordId",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Reviewer
}

impl Related<super::level_request::Entity> for Entity {
	fn to() -> RelationDef { Relation::LevelRequest.def() }
}

impl Related<super::reviewer::Entity> for Entity {
	fn to() -> RelationDef { Relation::Reviewer.def() }
}

impl ActiveModelBehavior for ActiveModel {}
