//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;

use super::sea_orm_active_enums::{LevelLength, RequestRating};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "level_request")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub level_id: u64,
	pub discord_id: u64,
	#[sea_orm(unique)]
	pub discord_message_id: Option<u64>,
	#[sea_orm(unique)]
	pub discord_thread_id: Option<u64>,
	pub name: String,
	pub author: String,
	pub request_rating: RequestRating,
	pub level_length: LevelLength,
	pub you_tube_video_link: String,
	pub has_requested_feedback: i8,
	pub notify: i8
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::moderator::Entity")]
	Moderator,
	#[sea_orm(has_many = "super::review::Entity")]
	Review,
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::DiscordId",
		to = "super::user::Column::DiscordId",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	User
}

impl Related<super::moderator::Entity> for Entity {
	fn to() -> RelationDef { Relation::Moderator.def() }
}

impl Related<super::review::Entity> for Entity {
	fn to() -> RelationDef { Relation::Review.def() }
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef { Relation::User.def() }
}

impl Related<super::reviewer::Entity> for Entity {
	fn to() -> RelationDef { super::review::Relation::Reviewer.def() }

	fn via() -> Option<RelationDef> { Some(super::review::Relation::LevelRequest.def().rev()) }
}

impl ActiveModelBehavior for ActiveModel {}
