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
		model::api::level_request_api::{LevelRequestApiResponse, LevelRequestApiResponseError},
		service::{level_request_service::LevelRequestService, request_service::RequestService}
	}
};
use crate::domain::model::api::level_request_api::LevelRequestApiRequest;

#[post("/request_level", format = "json", data = "<level_request_body>")]
pub async fn request_level<'a>(
	db_conn: &State<DatabaseConnection>,
	level_request_body: Json<LevelRequestApiRequest<'a>>
) -> Result<LevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let user_repository = MySqlUserRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();

	let level_request_service =
		LevelRequestService::new(level_request_repository, user_repository, gd_client);
	let request_rating = level_request_body.request_rating.into();
	match level_request_service
		.request(
			level_request_body.level_id,
			level_request_body.youtube_video_link.to_string(),
			level_request_body.discord_id,
			request_rating
		)
		.await
	{
		Ok(level_request_info) => {
			Ok(LevelRequestApiResponse{
				level_id: level_request_info.gd_level.level_id,
				discord_id: level_request_info.discord_user.discord_id,
				level_name: level_request_info.gd_level.name,
				level_author: level_request_info.gd_level.creator.name,
				request_score: level_request_info.request_rating.into(),
				youtube_video_link: level_request_info.youtube_video_link
			})
		}
		Err(level_request_error) => Err(level_request_error.into())
	}
}
