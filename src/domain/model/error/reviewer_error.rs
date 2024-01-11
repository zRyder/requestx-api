use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::api::reviewer_api::ReviewerApiResponseError;

#[derive(Debug, PartialEq)]
pub enum ReviewerError {
	DatabaseError(DbErr),
	ReviewerDoesNotExist
}

impl Display for ReviewerError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ReviewerError::DatabaseError(db_err) => {
				write!(
					f,
					"Unable to create reviewer due to database error: {}",
					db_err
				)
			}
			ReviewerError::ReviewerDoesNotExist => {
				write!(f, "Unable to get reviewer: Reviewer does not exist")
			}
		}
	}
}

impl Into<ReviewerApiResponseError> for ReviewerError {
	fn into(self) -> ReviewerApiResponseError {
		match self {
			ReviewerError::DatabaseError(_) => ReviewerApiResponseError::ReviewerError,
			ReviewerError::ReviewerDoesNotExist => ReviewerApiResponseError::ReviewerDoesNotExist
		}
	}
}

impl Error for ReviewerError {}
