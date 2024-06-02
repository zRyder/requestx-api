use std::{
	error::Error,
	fmt,
	fmt::{Debug, Display, Formatter}
};

use chrono::{DateTime, Duration, Utc};
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
	UserOnCooldown(DateTime<Utc>, Duration),
	EditUnownedLevelRequest(u64, u64, u64),
	LevelRequestsDisabled,
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
			LevelRequestError::UserOnCooldown(_last_request_time, _request_cooldown) => {
				write!(f, "The user is still on cooldown")
			}
			LevelRequestError::EditUnownedLevelRequest(
				_level_id,
				_discord_user_id,
				_requested_discord_user_id
			) => {
				write!(f, "The user attempted to edit a request they do not own.")
			}
			LevelRequestError::LevelRequestsDisabled => {
				write!(f, "Level requests are disabled")
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
			LevelRequestError::UserOnCooldown(last_request_time, request_cooldown) => {
				LevelRequestApiResponseError::UserOnCooldown(last_request_time, request_cooldown)
			}
			LevelRequestError::EditUnownedLevelRequest(
				level_id,
				discord_user_id,
				requested_discord_user_id
			) => LevelRequestApiResponseError::EditUnownedLevelRequest(
				level_id,
				discord_user_id,
				requested_discord_user_id
			),
			LevelRequestError::LevelRequestsDisabled => {
				LevelRequestApiResponseError::LevelRequetsDisabled
			}
			LevelRequestError::GeometryDashClientError(_, _) => {
				LevelRequestApiResponseError::LevelRequestError
			}
		}
	}
}
