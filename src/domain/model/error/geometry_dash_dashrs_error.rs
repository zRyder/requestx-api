use std::{
	error::Error,
	fmt,
	fmt::{write, Debug, Display, Formatter}
};

#[derive(Debug)]
pub enum GeometryDashDashrsError {
	HttpError(reqwest::Error),
	DashrsError(String),
	LevelNotFoundError(u64)
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
		}
	}
}

impl Error for GeometryDashDashrsError {}
