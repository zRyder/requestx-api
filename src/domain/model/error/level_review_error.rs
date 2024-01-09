use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::api::level_review_api::LevelReviewApiResponseError;

#[derive(Debug, PartialEq)]
pub enum LevelReviewError {
	DatabaseError(DbErr),
	LevelRequestDoesNotExist
}

impl Display for LevelReviewError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelReviewError::DatabaseError(db_err) => {
				write!(
					f,
					"Unable to submit review due to database error: {}",
					db_err
				)
			}
			LevelReviewError::LevelRequestDoesNotExist => {
				write!(
					f,
					"Unable to create level request: Level request does not exist"
				)
			}
		}
	}
}

impl Error for LevelReviewError {}

impl Into<LevelReviewApiResponseError> for LevelReviewError {
	fn into(self) -> LevelReviewApiResponseError {
		match self {
			LevelReviewError::DatabaseError(_) => LevelReviewApiResponseError::LevelReviewError,
			LevelReviewError::LevelRequestDoesNotExist => {
				LevelReviewApiResponseError::LevelRequestDoesNotExist
			}
		}
	}
}
