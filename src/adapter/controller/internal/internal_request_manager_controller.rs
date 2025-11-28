use rocket_framework::{futures::io, serde::json::Json};

use crate::domain::{
	model::{
		api::auth_api::Auth,
		internal::api::internal_request_manager_api::{
			InternalUpdateRequestConfigApiRequest, InternalUpdateRequestConfigApiResponse
		}
	},
	service::internal::request_manager_service::RequestManagerService
};

#[get("/request_config")]
pub async fn get_request_config() -> io::Result<InternalGetRequestConfigApiResponse> {
	let request_manager_service = RequestManagerService {};

	let response = InternalGetRequestConfigApiResponse {
		enable_requests: request_manager_service.get_enable_request().await,
		enable_gd_requests: request_manager_service.get_enable_gd_request().await,
		duration_in_minutes: request_manager_service
			.get_request_cooldown()
			.await
			.num_minutes()
			.unsigned_abs(),
		allow_non_user_created_levels: request_manager_service
			.get_allow_non_user_created_levels()
			.await
	};

	Ok(response)
}

#[patch(
	"/request_config",
	format = "json",
	data = "<update_request_config_body>"
)]
pub async fn update_request_cooldown<'a>(
	update_request_config_body: Json<InternalUpdateRequestConfigApiRequest>,
	_auth: Auth
) -> io::Result<InternalUpdateRequestConfigApiResponse> {
	let request_manager_service = RequestManagerService {};

	if let Some(duration_in_minutes) = update_request_config_body.duration_in_minutes {
		request_manager_service
			.set_request_cooldown(duration_in_minutes)
			.await;
	}

	if let Some(enable_requests) = update_request_config_body.enable_requests {
		request_manager_service
			.set_enable_request(enable_requests)
			.await;
	}

	if let Some(enable_gd_requests) = update_request_config_body.enable_gd_requests {
		request_manager_service
			.set_enable_gd_request(enable_gd_requests)
			.await;
	}

	if let Some(allow_non_user_created_levels) =
		update_request_config_body.allow_non_user_created_levels
	{
		request_manager_service
			.set_allow_non_user_created_levels(allow_non_user_created_levels)
			.await;
	}
	request_manager_service.update_client_config_file().await;

	Ok(InternalUpdateRequestConfigApiResponse {})
}
