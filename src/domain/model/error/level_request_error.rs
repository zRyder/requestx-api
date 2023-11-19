use std::{
	error::Error,
	fmt,
	fmt::{write, Debug, Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::error::geometry_dash_dashrs_error::GeometryDashDashrsError;

#[derive(Debug, PartialEq)]
pub enum LevelRequestError {
	DatabaseError(DbErr),
	LevelRequestExists,
	GeometryDashClientError(u64, GeometryDashDashrsError)
}

impl Display for LevelRequestError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			LevelRequestError::DatabaseError(db_err) => {
				write!(
					f,
					"Unable to create level request due to MySQL DB error: {}",
					db_err
				)
			}
			LevelRequestError::LevelRequestExists => {
				write!(f, "Unable to create level request due to conflict: Level has already been requested ")
			}
			LevelRequestError::GeometryDashClientError(level_id, client_error) => {
				write!(
					f,
					"Unable to get Geometry Dash level info for level {}: {}",
					level_id, client_error
				)
			}
		}
	}
}

impl Error for LevelRequestError {}
