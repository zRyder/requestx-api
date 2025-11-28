use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use sea_orm::DbErr;

use crate::domain::model::api::user_api::DiscordUserApiResponseError;

#[derive(Debug, PartialEq)]
pub enum DiscordError {
	UserDoesNotExist,
	GDAccountDoesNotExist(String),
	GDAccountLinkExpired,
	DiscordAccountAlreadyLinked,
	InvalidGDAccountLinkToken,
	DatabaseError(DbErr),
	DiscordError
}

impl Display for DiscordError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DiscordError::UserDoesNotExist => {
				write!(f, "User does not exist")
			}
			DiscordError::GDAccountDoesNotExist(account_name) => {
				write!(f, "GD account with name {} does not exist", account_name)
			}
			DiscordError::GDAccountLinkExpired => {
				write!(f, "GD account link has expired")
			}
			DiscordError::DiscordAccountAlreadyLinked => {
				write!(f, "Discord account is already linked to a GD account")
			}
			DiscordError::InvalidGDAccountLinkToken => {
				write!(f, "Invalid GD account link token provided")
			}
			DiscordError::DatabaseError(db_err) => {
				write!(f, "Database error {}", db_err)
			}
			DiscordError::DiscordError => {
				write!(f, "Discord error")
			}
		}
	}
}

impl Error for DiscordError {}

impl Into<DiscordUserApiResponseError> for DiscordError {
	fn into(self) -> DiscordUserApiResponseError {
		match self {
			DiscordError::UserDoesNotExist => DiscordUserApiResponseError::UserDoesNotExist,
			DiscordError::GDAccountDoesNotExist(gd_username) => {
				DiscordUserApiResponseError::GDAccountDoesNotExist(gd_username)
			}
			DiscordError::GDAccountLinkExpired => DiscordUserApiResponseError::GDAccountLinkExpired,
			DiscordError::DiscordAccountAlreadyLinked => {
				DiscordUserApiResponseError::DiscordAccountAlreadyLinked
			}
			DiscordError::InvalidGDAccountLinkToken => {
				DiscordUserApiResponseError::InvalidGDAccountLinkToken
			}
			DiscordError::DatabaseError(_) => DiscordUserApiResponseError::DiscordUserError,
			DiscordError::DiscordError => DiscordUserApiResponseError::DiscordUserError
		}
	}
}