use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			level_request_repository::LevelRequestRepository,
			moderator_repository::ModeratorRepository,
		},
	},
	domain::{
		model::{
			api::{auth_api::Auth, level_request_api::PostSendLevelRequestApiResponse},
			internal::api::moderator_api::{ModeratorApiResponseError, PostModeratorApiRequest},
		},
		service::moderator_service::ModeratorService,
	},
};

#[post("/send_level", format = "json", data = "<send_level_body>")]
pub async fn send_level<'a>(
	db_conn: &State<DatabaseConnection>,
	send_level_body: Json<PostModeratorApiRequest>,
	_auth: Auth,
) -> Result<PostSendLevelRequestApiResponse, ModeratorApiResponseError> {
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let moderator_repository = ModeratorRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();
	let moderator_service =
		ModeratorService::new(&moderator_repository, &level_request_repository, &gd_client);

	match moderator_service
		.send_level(
			send_level_body.level_id,
			send_level_body.suggested_rating.into(),
			send_level_body.suggested_score.into(),
		)
		.await
	{
		Ok(send_level_request_data) => Ok(PostSendLevelRequestApiResponse::from(
			send_level_request_data,
		)),
		Err(send_level_error) => Err(send_level_error.into()),
	}
}
