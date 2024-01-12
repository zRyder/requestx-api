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
		model::api::level_request_api::{GetLevelRequestApiResponse, LevelRequestApiResponseError,
			PostLevelRequestApiRequest, PostLevelRequestApiResponse
		},
		service::{level_request_service::LevelRequestService, request_service::RequestService}
	}
};
use crate::domain::model::api::auth_api::Auth;

#[get("/request_level/<level_id>")]
pub async fn get_level_request(
	db_conn: &State<DatabaseConnection>,
	level_id: u64,
	_auth: Auth
) -> Result<GetLevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let user_repository = MySqlUserRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();

	let level_request_service =
		LevelRequestService::new(level_request_repository, user_repository, gd_client);

	match level_request_service
		.get_level_request(level_id)
		.await
	{
		Ok(level_request_info) => Ok(GetLevelRequestApiResponse::from(level_request_info)),
		Err(get_level_request_error) => Err(get_level_request_error.into())
	}
}

#[post("/request_level", format = "json", data = "<level_request_body>")]
pub async fn request_level<'a>(
	db_conn: &State<DatabaseConnection>,
	level_request_body: Json<PostLevelRequestApiRequest<'a>>,
	_auth: Auth
) -> Result<PostLevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let user_repository = MySqlUserRepository::new(db_conn);
	let gd_client = GeometryDashDashrsClient::new();

	let level_request_service =
		LevelRequestService::new(level_request_repository, user_repository, gd_client);
	let request_rating = level_request_body.request_rating.into();
	match level_request_service
		.make_level_request(
			level_request_body.level_id,
			level_request_body.youtube_video_link.to_string(),
			level_request_body.discord_id,
			request_rating,
			level_request_body.has_requested_feedback
		)
		.await
	{
		Ok(level_request_info) => Ok(PostLevelRequestApiResponse::from(level_request_info)),
		Err(level_request_error) => Err(level_request_error.into())
	}
}
