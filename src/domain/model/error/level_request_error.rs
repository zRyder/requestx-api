use std::{
	error::Error,
	fmt,
	fmt::{write, Debug, Display, Formatter}
};

use sea_orm::DbErr;

#[derive(Debug, PartialEq)]
pub enum LevelRequestError {
	DatabaseError(DbErr),
	LevelRequestExists
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
		}
	}
}

impl Error for LevelRequestError {}
