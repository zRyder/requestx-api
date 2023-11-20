use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_dashrs_client::GeometryDashDashrsClient,
		mysql::mysql_level_request_repository::MySqlLevelRequestRepository
	},
	domain::{
		model::api::{
			level_request_api_request::LevelRequestApiRequest,
			level_request_api_response::{LevelRequestApiResponse, LevelRequestApiResponseError}
		},
		service::{level_request_service::LevelRequestService, request_service::RequestService}
	}
};

#[post("/request_level", format = "json", data = "<level_request_body>")]
pub async fn request_level(
	db_conn: &State<DatabaseConnection>,
	level_request_body: Json<LevelRequestApiRequest<'_>>
) -> Result<LevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();

	let level_request_service = LevelRequestService::new(level_request_repository, gd_client);
	let request_rating = level_request_body.request_rating.into();
	match level_request_service
		.request(
			level_request_body.level_id,
			level_request_body.youtube_video_link,
			level_request_body.discord_id,
			request_rating
		)
		.await
	{
		Ok(()) => Ok(LevelRequestApiResponse {}),
		Err(level_request_error) => Err(level_request_error.into())
	}
}
