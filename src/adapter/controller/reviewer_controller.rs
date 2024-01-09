use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::mysql::mysql_reviewer_repository::MySqlReviewerRepository,
	domain::{
		model::api::reviewer_api::{
			CreateReviewerApiRequest, GetReviewerApiResponse, ReviewerApiResponseError
		},
		service::{
			level_reviewer_service::LevelReviewerService, reviewer_service::ReviewerService
		}
	}
};

#[get("/reviewer/<reviewer_discord_id>?<is_active>")]
pub async fn get_reviewer(
	db_conn: &State<DatabaseConnection>,
	reviewer_discord_id: u64,
	is_active: bool
) -> Result<GetReviewerApiResponse, ReviewerApiResponseError> {
	let reviewer_repository = MySqlReviewerRepository::new(&db_conn);
	let reviewer_service = LevelReviewerService::new(reviewer_repository);

	match reviewer_service
		.get_reviewer(reviewer_discord_id, is_active)
		.await
	{
		Ok(reviewer) => Ok(GetReviewerApiResponse::from(reviewer)),
		Err(get_reviewer_error) => Err(get_reviewer_error.into())
	}
}

#[post("/reviewer", format = "json", data = "<create_reviewer_api_request>")]
pub async fn create_reviewer(
	db_conn: &State<DatabaseConnection>,
	create_reviewer_api_request: Json<CreateReviewerApiRequest>
) -> Result<(), ReviewerApiResponseError> {
	let reviewer_repository = MySqlReviewerRepository::new(&db_conn);
	let reviewer_service = LevelReviewerService::new(reviewer_repository);

	match reviewer_service
		.create_reviewer(create_reviewer_api_request.reviewer_discord_id)
		.await
	{
		Ok(()) => Ok(()),
		Err(create_reviewer_error) => Err(create_reviewer_error.into())
	}
}

#[delete("/reviewer/<reviewer_discord_id>")]
pub async fn remove_reviewer(
	db_conn: &State<DatabaseConnection>,
	reviewer_discord_id: u64
) -> Result<(), ReviewerApiResponseError> {
	let reviewer_repository = MySqlReviewerRepository::new(&db_conn);
	let reviewer_service = LevelReviewerService::new(reviewer_repository);

	match reviewer_service.remove_reviewer(reviewer_discord_id).await {
		Ok(()) => Ok(()),
		Err(create_reviewer_error) => Err(create_reviewer_error.into())
	}
}
