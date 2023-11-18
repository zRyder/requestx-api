//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "request_rating")]
pub enum RequestRating {
	#[sea_orm(string_value = "easy")]
	Easy,
	#[sea_orm(string_value = "normal")]
	Normal,
	#[sea_orm(string_value = "hard")]
	Hard,
	#[sea_orm(string_value = "harder")]
	Harder,
	#[sea_orm(string_value = "insane")]
	Insane,
	#[sea_orm(string_value = "demon")]
	Demon
}
