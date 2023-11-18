use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::mysql::mysql_level_request_repository::MySqlLevelRequestRepository,
	domain::{
		model::api::level_request_api_request::LevelRequestApiRequest,
		service::request_service::RequestService
	}
};

#[get("/", data = "<level_request_body>")]
pub async fn request_level(
	db_conn: &State<DatabaseConnection>,
	level_request_body: Json<LevelRequestApiRequest>
) {
	let repository = MySqlLevelRequestRepository::new(db_conn);

	// let level_request_service = LevelRequestService {
	//     repository
	// };
}
