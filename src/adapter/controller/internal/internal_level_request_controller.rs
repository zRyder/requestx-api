use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_dashrs_client::GeometryDashDashrsClient,
		mysql::{
			mysql_level_request_repository::MySqlLevelRequestRepository,
			mysql_user_repository::MySqlUserRepository
		}
	},
	domain::{
		model::{
			api::{auth_api::Auth, level_request_api::LevelRequestApiResponseError},
			internal::api::internal_level_request_api::{
				InternalUpdateLevelRequestDiscordDataApiResponse,
				InternalUpdateLevelRequestMessageIdApiRequest,
				InternalUpdateLevelRequestThreadIdApiRequest
			}
		},
		service::{level_request_service::LevelRequestService, request_service::RequestService}
	}
};

#[patch(
	"/request_level",
	format = "json",
	data = "<update_level_request_message_id_body>",
	rank = 1
)]
pub async fn update_level_request_message_id<'a>(
	db_conn: &State<DatabaseConnection>,
	update_level_request_message_id_body: Json<InternalUpdateLevelRequestMessageIdApiRequest>,
	_auth: Auth
) -> Result<InternalUpdateLevelRequestDiscordDataApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let user_repository = MySqlUserRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();
	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	match level_request_service
		.update_level_request_message_id(
			update_level_request_message_id_body.level_id,
			update_level_request_message_id_body.discord_message_id
		)
		.await
	{
		Ok(()) => Ok(InternalUpdateLevelRequestDiscordDataApiResponse {}),
		Err(update_level_request_error) => Err(update_level_request_error.into())
	}
}

#[patch(
	"/request_level/thread",
	format = "json",
	data = "<update_level_request_thread_id_body>"
)]
pub async fn update_level_request_thread_id<'a>(
	db_conn: &State<DatabaseConnection>,
	update_level_request_thread_id_body: Json<InternalUpdateLevelRequestThreadIdApiRequest>,
	_auth: Auth
) -> Result<InternalUpdateLevelRequestDiscordDataApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let user_repository = MySqlUserRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();
	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	match level_request_service
		.update_level_request_thread_id(
			update_level_request_thread_id_body.level_id,
			update_level_request_thread_id_body.discord_thread_id
		)
		.await
	{
		Ok(()) => Ok(InternalUpdateLevelRequestDiscordDataApiResponse {}),
		Err(update_level_request_error) => Err(update_level_request_error.into())
	}
}
