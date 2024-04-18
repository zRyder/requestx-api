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
		request_manager_service.set_request_cooldown(duration_in_minutes);
	}

	if let Some(enable_requests) = update_request_config_body.enable_requests {
		request_manager_service.set_enable_request(enable_requests)
	}

	if let Some(enable_gd_requests) = update_request_config_body.enable_gd_requests {
		request_manager_service.set_enable_gd_request(enable_gd_requests)
	}

	Ok(InternalUpdateRequestConfigApiResponse {})
}
