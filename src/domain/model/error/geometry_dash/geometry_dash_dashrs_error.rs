use std::{
	error::Error,
	fmt,
	fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum GeometryDashDashrsError {
	HttpError(reqwest::Error),
	DashrsError(String),
	LevelNotFoundError(u64),
	UserNotFoundError(String),
	LevelAlreadyRated(u64),
	NoProfileCommentsFound,
}

impl Display for GeometryDashDashrsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			GeometryDashDashrsError::HttpError(reqwest_err) => {
				write!(
					f,
					"Unable to make call to url {} with error: {}",
					reqwest_err.url().unwrap(),
					reqwest_err
				)
			}
			GeometryDashDashrsError::DashrsError(dashrs_err) => {
				write!(
					f,
					"Unable to process response from Boomlings: {}",
					dashrs_err
				)
			}
			GeometryDashDashrsError::LevelNotFoundError(level_id) => {
				write!(f, "Unable to find level with level ID: {}", level_id)
			}
			GeometryDashDashrsError::UserNotFoundError(gd_username) => {
				write!(f, "Unable to find user with username: {}", gd_username)
			}
			GeometryDashDashrsError::LevelAlreadyRated(level_id) => {
				write!(f, "Level with ID: {} has already been rated", level_id)
			}
			GeometryDashDashrsError::NoProfileCommentsFound => {
				write!(f, "User has no profile comments")
			}
		}
	}
}

impl Error for GeometryDashDashrsError {}

impl PartialEq for GeometryDashDashrsError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::HttpError(_), Self::HttpError(_)) => true,
			(Self::DashrsError(s1), Self::DashrsError(s2)) => s1 == s2,
			(Self::LevelNotFoundError(n1), Self::LevelNotFoundError(n2)) => n1 == n2,
			(Self::UserNotFoundError(n1), Self::UserNotFoundError(n2)) => n1 == n2,
			_ => false,
		}
	}
}
