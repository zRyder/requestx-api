use dash_rs::{request::level::LevelRequest, response::parse_get_gj_levels_response};
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
		gd_level::GDLevel
	},
	rocket::common::constants::{APPLICATION_FORM_URL_ENCODED, CONTENT_TYPE}
};

pub struct GeometryDashDashrsClient {
	client: Client
}

impl GeometryDashClient for GeometryDashDashrsClient {
	async fn get_gd_level_info(self, level_id: u64) -> Result<GDLevel, GeometryDashDashrsError> {
		let get_level_info_request = LevelRequest::new(level_id);

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
					Ok(gd_level_info) => match gd_level_info.first() {
						Some(matched_level) => Ok(GDLevel::from(matched_level)),
						None => Err(LevelNotFoundError(level_id))
					},
					Err(dashrs_error) => Err(DashrsError(dashrs_error.to_string()))
				}
			}
			Err(request_err) => Err(HttpError(request_err))
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
