use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			level_request_repository::LevelRequestRepository, user_repository::UserRepository
		}
	},
	domain::{
		model::{
			api::{auth_api::Auth, level_request_api::LevelRequestApiResponseError},
			internal::api::internal_level_request_api::{
				InternalUpdateLevelRequestDiscordDataApiResponse,
				InternalUpdateLevelRequestMessageIdApiRequest
			}
		},
		service::level_request_service::LevelRequestService
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
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let user_repository = UserRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();
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
