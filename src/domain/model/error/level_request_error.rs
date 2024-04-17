use std::{
	error::Error,
	fmt,
	fmt::{Debug, Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::{
	api::level_request_api::LevelRequestApiResponseError,
	error::geometry_dash_dashrs_error::GeometryDashDashrsError
};

#[derive(Debug, PartialEq)]
pub enum LevelRequestError {
	MalformedRequest,
	DatabaseError(DbErr),
	LevelRequestExists,
	LevelRequestDoesNotExist,
	UserOnCooldown,
	GeometryDashClientError(u64, GeometryDashDashrsError)
}

impl Display for LevelRequestError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			LevelRequestError::MalformedRequest => {
				write!(f, "Level request is malformed")
			}
			LevelRequestError::DatabaseError(db_err) => {
				write!(
					f,
					"Unable to create level request due to MySQL DB error: {}",
					db_err
				)
			}
			LevelRequestError::LevelRequestExists => {
				write!(f, "Unable to create level request due to conflict: Level has already been requested")
			}
			LevelRequestError::LevelRequestDoesNotExist => {
				write!(
					f,
					"Unable to update level request due to conflict: Level request does not exist"
				)
			}
			LevelRequestError::UserOnCooldown => {
				write!(f, "The user is still on cooldown")
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

impl Into<LevelRequestApiResponseError> for LevelRequestError {
	fn into(self) -> LevelRequestApiResponseError {
		match self {
			LevelRequestError::MalformedRequest => LevelRequestApiResponseError::MalformedRequest,
			LevelRequestError::DatabaseError(_) => LevelRequestApiResponseError::LevelRequestError,
			LevelRequestError::LevelRequestExists => {
				LevelRequestApiResponseError::LevelRequestExists
			}
			LevelRequestError::LevelRequestDoesNotExist => {
				LevelRequestApiResponseError::LevelRequestDoesNotExist
			}
			LevelRequestError::UserOnCooldown => LevelRequestApiResponseError::UserOnCooldown,
			LevelRequestError::GeometryDashClientError(_, _) => {
				LevelRequestApiResponseError::LevelRequestError
			}
		}
	}
}
