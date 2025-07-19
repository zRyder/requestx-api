use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::api::user_api::DiscordUserApiResponseError;

#[derive(Debug, PartialEq)]
pub enum DiscordError {
	UserDoesNotExist,
	DatabaseError(DbErr)
}

impl Display for DiscordError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DiscordError::UserDoesNotExist => {
				write!(f, "User does not exist")
			}
			DiscordError::DatabaseError(db_err) => {
				write!(f, "Database error {}", db_err)
			}
		}
	}
}

impl Error for DiscordError {}

impl Into<DiscordUserApiResponseError> for DiscordError {
	fn into(self) -> DiscordUserApiResponseError {
		match self {
			DiscordError::UserDoesNotExist => DiscordUserApiResponseError::UserDoesNotExist,
			DiscordError::DatabaseError(_) => DiscordUserApiResponseError::DiscordUserError
		}
	}
}
