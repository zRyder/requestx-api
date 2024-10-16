use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_dashrs_client::GeometryDashDashrsClient,
		mysql::{
			mysql_level_request_repository::MySqlLevelRequestRepository,
			mysql_review_repository::MySqlReviewRepository,
			mysql_user_repository::MySqlUserRepository
		}
	},
	domain::{
		model::{
			api::{auth_api::Auth, level_review_api::LevelReviewApiResponseError},
			internal::api::internal_level_review_api::{
				InternalUpdateLevelReviewDiscordDataApiResponse,
				InternalUpdateLevelReviewMessageIdApiRequest
			}
		},
		service::{
			level_request_service::LevelRequestService, level_review_service::LevelReviewService,
			review_service::ReviewService
		}
	}
};

#[patch(
	"/review_level",
	format = "json",
	data = "<update_level_review_message_id_body>",
	rank = 1
)]
pub async fn update_level_review_message_id<'a>(
	db_conn: &State<DatabaseConnection>,
	update_level_review_message_id_body: Json<InternalUpdateLevelReviewMessageIdApiRequest>,
	_auth: Auth
) -> Result<InternalUpdateLevelReviewDiscordDataApiResponse, LevelReviewApiResponseError> {
	let level_review_repository = MySqlReviewRepository::new(db_conn);
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let user_repository = MySqlUserRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();
	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	let level_review_service =
		LevelReviewService::new(&level_review_repository, &level_request_service);
	match level_review_service
		.update_level_request_thread_id(
			update_level_review_message_id_body.level_id,
			update_level_review_message_id_body.discord_id,
			update_level_review_message_id_body.discord_message_id
		)
		.await
	{
		Ok(()) => Ok(InternalUpdateLevelReviewDiscordDataApiResponse {}),
		Err(update_level_review_error) => Err(update_level_review_error.into())
	}
}
