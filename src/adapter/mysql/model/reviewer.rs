//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "reviewer")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub discord_id: u64,
	pub active: i8
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::review::Entity")]
	Review
}

impl Related<super::review::Entity> for Entity {
	fn to() -> RelationDef { Relation::Review.def() }
}

impl Related<super::level_request::Entity> for Entity {
	fn to() -> RelationDef { super::review::Relation::LevelRequest.def() }

	fn via() -> Option<RelationDef> { Some(super::review::Relation::Reviewer.def().rev()) }
}

impl ActiveModelBehavior for ActiveModel {}
