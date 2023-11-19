use rocket_framework::{http::Status, serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_dashrs_client::GeometryDashDashrsClient,
		mysql::mysql_level_request_repository::MySqlLevelRequestRepository
	},
	domain::{
		model::{
			api::{
				level_request_api_request::LevelRequestApiRequest,
				level_request_api_response::LevelRequestApiResponse
			},
			error::level_request_error::LevelRequestError
		},
		service::{level_request_service::LevelRequestService, request_service::RequestService}
	}
};

#[post("/request_level", format = "json", data = "<level_request_body>")]
pub async fn request_level(
	db_conn: &State<DatabaseConnection>,
	level_request_body: Json<LevelRequestApiRequest<'_>>
) -> LevelRequestApiResponse {
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
		Ok(()) => LevelRequestApiResponse {
			json: Json("Created".to_string()),
			status: Status::Created
		},
		Err(level_request_error) => handle_level_request_error(level_request_error)
	}
}

fn handle_level_request_error(level_request_error: LevelRequestError) -> LevelRequestApiResponse {
	match level_request_error {
		LevelRequestError::DatabaseError(_db_error) => LevelRequestApiResponse {
			json: Json("Internal Server Error (Database)".to_string()),
			status: Status::InternalServerError
		},
		LevelRequestError::LevelRequestExists => LevelRequestApiResponse {
			json: Json("Level has already been requested".to_string()),
			status: Status::Conflict
		},
		LevelRequestError::GeometryDashClientError(_level_id, _inner_error) => {
			LevelRequestApiResponse {
				json: Json("Internal Server Error (Geometry Dash)".to_string()),
				status: Status::InternalServerError
			}
		}
	}
}
