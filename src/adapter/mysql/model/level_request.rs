//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;

use super::sea_orm_active_enums::RequestRating;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "level_request")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: u64,
	pub name: String,
	pub description: Option<String>,
	pub author: String,
	pub request_rating: RequestRating
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}