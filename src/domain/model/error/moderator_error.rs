use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::internal::api::moderator_api::ModeratorApiResponseError;

#[derive(Debug, PartialEq)]
pub enum ModeratorError {
	DatabaseError(DbErr),
	LevelRequestDoesNotExists,
	UnsendableLevel,
	GeometryDashDashrsError
}

impl Display for ModeratorError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ModeratorError::DatabaseError(db_err) => {
				write!(f, "Unable to send level due to database error: {}", db_err)
			}
			ModeratorError::LevelRequestDoesNotExists => {
				write!(f, "Level request does not exist")
			}
			ModeratorError::UnsendableLevel => {
				write!(f, "Level could not be sent")
			}
			ModeratorError::GeometryDashDashrsError => {
				write!(f, "Error calling Geometry Dash")
			}
		}
	}
}

impl Into<ModeratorApiResponseError> for ModeratorError {
	fn into(self) -> ModeratorApiResponseError {
		match self {
			ModeratorError::DatabaseError(_) => ModeratorApiResponseError::ModeratorError,
			ModeratorError::LevelRequestDoesNotExists => {
				ModeratorApiResponseError::LevelRequestDoesNotExist
			}
			ModeratorError::UnsendableLevel => ModeratorApiResponseError::UnsendableLevel,
			ModeratorError::GeometryDashDashrsError => ModeratorApiResponseError::ModeratorError
		}
	}
}

impl Error for ModeratorError {}
