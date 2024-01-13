use dash_rs::{
	request::{account::LoginRequest, level::LevelsRequest, moderator::SuggestStarsRequest},
	response::parse_get_gj_levels_response
};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client
};

use crate::{
	adapter::geometry_dash::geometry_dash_client::GeometryDashClient,
	domain::model::{
		error::geometry_dash_dashrs_error::{
			GeometryDashDashrsError,
			GeometryDashDashrsError::{DashrsError, HttpError, LevelNotFoundError}
		},
		gd_level::GDLevel,
		moderator::Moderator
	},
	rocket::common::{
		constants::{APPLICATION_FORM_URL_ENCODED, CONTENT_TYPE}
	}
};
use crate::rocket::common::config::geometry_dash_config::GEOMETRY_DASH_CONFIG;

pub struct GeometryDashDashrsClient {
	client: Client
}

impl GeometryDashClient for GeometryDashDashrsClient {
	async fn get_gd_level_info(&self, level_id: u64) -> Result<GDLevel, GeometryDashDashrsError> {
		let level_id_str = &level_id.to_string();
		let get_level_info_request = LevelsRequest::default().search(level_id_str);

		debug!("Calling Geometry Dash servers for level {}", level_id);
		let raw_response_result = self
			.client
			.post(get_level_info_request.to_url())
			.body(get_level_info_request.to_string())
			.send()
			.await;

		match raw_response_result {
			Ok(raw_response) => {
				let parsed_response = raw_response.text().await.unwrap();

				let gd_level_info_result = parse_get_gj_levels_response(&parsed_response);
				match gd_level_info_result {
					Ok(gd_level_info) => {
						debug!(
							"Successfully called Geometry Dash servers for level {}",
							level_id
						);
						match gd_level_info.first() {
							Some(matched_level) => Ok(GDLevel::from(matched_level)),
							None => Err(LevelNotFoundError(level_id))
						}
					}
					Err(dashrs_error) => {
						error!(
							"Error parsing response from Geometry Dash servers: {}",
							dashrs_error
						);
						Err(DashrsError(dashrs_error.to_string()))
					}
				}
			}
			Err(request_err) => {
				error!("Error calling Geometry Dash servers: {}", request_err);
				Err(HttpError(request_err))
			}
		}
	}

	async fn send_gd_level(
		&self,
		moderator_request: Moderator
	) -> Result<(), GeometryDashDashrsError> {
		match LoginRequest::default()
			.user_name(&GEOMETRY_DASH_CONFIG.gd_username)
			.password(&GEOMETRY_DASH_CONFIG.gd_password)
			.to_authenticated_user()
			.await
		{
			Ok(auth_user) => {
				let send_level_request =
					SuggestStarsRequest::new(auth_user, moderator_request.level_id)
						.feature(moderator_request.suggested_rating.into())
						.stars(moderator_request.suggested_score.into());

				debug!(
					"Calling Geometry Dash servers for sending level {:?}",
					&moderator_request
				);
				let raw_response_result = self
					.client
					.post(send_level_request.to_url())
					.body(send_level_request.to_string())
					.send()
					.await;

				match raw_response_result {
					Ok(raw_response) => {
						let parsed_response = raw_response.text().await.unwrap();

						if parsed_response.eq("1") {
							Ok(())
						} else {
							Err(DashrsError("-1".to_string()))
						}
					}
					Err(request_err) => {
						error!("Error calling Geometry Dash servers: {}", request_err);
						Err(HttpError(request_err))
					}
				}
			}
			Err(auth_error) => Err(DashrsError(auth_error.to_string()))
		}
	}
}

impl GeometryDashDashrsClient {
	pub fn new() -> Self {
		let mut default_headers = HeaderMap::new();
		default_headers.insert(
			CONTENT_TYPE,
			HeaderValue::from_static(APPLICATION_FORM_URL_ENCODED)
		);
		GeometryDashDashrsClient {
			client: Client::builder()
				.default_headers(default_headers)
				.build()
				.expect("Client::new")
		}
	}
}
