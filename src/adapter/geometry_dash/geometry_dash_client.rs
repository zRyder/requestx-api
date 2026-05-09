use std::borrow::Cow;

use dash_rs::{
	request::{
		account::AuthenticatedUser, comment::ProfileCommentsRequest, level::LevelsRequest,
		moderator::SuggestStarsRequest, user::UserSearchRequest,
	},
	response::{
		parse_get_gj_acccount_comments_response, parse_get_gj_levels_response,
		parse_get_gj_users_response, ResponseError,
	},
};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client,
};

use crate::{
	domain::model::{
		error::geometry_dash::geometry_dash_dashrs_error::{
			GeometryDashDashrsError,
			GeometryDashDashrsError::{
				DashrsError, HttpError, LevelAlreadyRated, LevelNotFoundError,
				NoProfileCommentsFound, UserNotFoundError,
			},
		},
		level_request::GDLevel,
		moderator::{Moderator, SuggestedScore},
	},
	rocket::common::{
		config::common_config::APP_CONFIG,
		constants::{APPLICATION_FORM_URL_ENCODED, CONTENT_TYPE},
	},
};

pub struct GeometryDashClient {
	client: Client,
}

#[cfg_attr(test, mockall::automock)]
impl GeometryDashClient {
	pub fn new() -> Self {
		let mut default_headers = HeaderMap::new();
		default_headers.insert(
			CONTENT_TYPE,
			HeaderValue::from_static(APPLICATION_FORM_URL_ENCODED),
		);
		GeometryDashClient {
			client: Client::builder()
				.default_headers(default_headers)
				.build()
				.expect("Client::new"),
		}
	}

	pub async fn get_gd_level_info(
		&self,
		level_id: u64,
	) -> Result<GDLevel, GeometryDashDashrsError> {
		let level_id_str = &level_id.to_string();
		let get_level_info_request = LevelsRequest::default().search(level_id_str);

		info!("Calling Geometry Dash servers for level {}", level_id);
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
							None => Err(LevelNotFoundError(level_id)),
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

	pub async fn query_gd_player_and_account_id(
		&self,
		gd_username: &str,
	) -> Result<(u64, u64), GeometryDashDashrsError> {
		let search_gd_player_request = UserSearchRequest::new(gd_username);

		info!(
			"Calling Geometry Dash servers for user search {}",
			gd_username
		);
		let raw_response_result = self
			.client
			.post(search_gd_player_request.to_url())
			.body(search_gd_player_request.to_string())
			.send()
			.await;

		match raw_response_result {
			Ok(raw_response) => {
				let parsed_response = raw_response.text().await.unwrap();
				let user_search_result = parse_get_gj_users_response(&parsed_response);
				match user_search_result {
					Ok(searched_user) => {
						debug!(
							"Successfully called Geometry Dash servers for user {}",
							gd_username
						);
						Ok((searched_user.user_id, searched_user.account_id))
					}
					Err(dashrs_error) => {
						if matches!(dashrs_error, ResponseError::NotFound) {
							error!("Could not find user with username: {}", gd_username);
							return Err(UserNotFoundError(gd_username.to_string()));
						}
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

	pub async fn send_gd_level(
		&self,
		moderator_request: Moderator,
	) -> Result<(), GeometryDashDashrsError> {
		let auth_user = AuthenticatedUser::new(
			&APP_CONFIG.get().unwrap().geometry_dash_config.gd_username,
			57903,
			Cow::from(&APP_CONFIG.get().unwrap().geometry_dash_config.gd_password),
		);
		let send_level_request = SuggestStarsRequest::new(auth_user, moderator_request.level_id)
			.feature(moderator_request.suggested_rating.into())
			.stars(moderator_request.suggested_score.into());

		info!(
			"Calling Geometry Dash servers for sending level {:?}",
			&moderator_request
		);

		match self.is_rated(moderator_request.level_id).await {
			Ok(is_rated) => {
				if is_rated && moderator_request.suggested_score != SuggestedScore::Rated {
					return Err(LevelAlreadyRated(moderator_request.level_id));
				}
			}
			Err(gd_error) => {
				error!("Error calling Geometry Dash servers: {}", gd_error);
				return Err(gd_error);
			}
		}

		let raw_response_result = self
			.client
			.post(send_level_request.to_url())
			.body(send_level_request.to_string())
			.header(CONTENT_TYPE, APPLICATION_FORM_URL_ENCODED)
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

	pub async fn get_gd_public_account_token(
		&self,
		account_id: u64,
	) -> Result<String, GeometryDashDashrsError> {
		let get_gd_account_comments_request = ProfileCommentsRequest::new(account_id);

		info!(
			"Calling Geometry Dash servers for user account comments {}",
			account_id
		);
		let raw_response_result = self
			.client
			.post(get_gd_account_comments_request.to_url())
			.body(get_gd_account_comments_request.to_string())
			.send()
			.await;

		match raw_response_result {
			Ok(raw_response) => {
				let parsed_response = raw_response.text().await.unwrap();
				let user_account_comments_result =
					parse_get_gj_acccount_comments_response(&parsed_response);

				match user_account_comments_result {
					Ok(account_comments_list) => {
						if let Some(account_comment) = account_comments_list.first() {
							let encoded_account_comment =
								account_comment.content.as_ref().unwrap().to_owned();
							match &encoded_account_comment.into_processed() {
								Ok(decoded_comment) => Ok(decoded_comment.0.parse().unwrap()),
								Err(thunk_processing_error) => {
									error!("Error decoding comment: {}", thunk_processing_error);
									Err(DashrsError(thunk_processing_error.to_string()))
								}
							}
						} else {
							warn!("No account comments found for user: {}", account_id);
							Err(NoProfileCommentsFound)
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

	pub async fn is_rated(&self, level_id: u64) -> Result<bool, GeometryDashDashrsError> {
		let level_id_str = &level_id.to_string();
		let get_level_info_request = LevelsRequest::default().search(level_id_str);

		info!("Calling Geometry Dash servers for level {}", level_id);
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
							Some(matched_level) => Ok(matched_level.stars != 0),
							None => Err(LevelNotFoundError(level_id)),
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
}
