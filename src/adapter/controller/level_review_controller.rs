use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{
	adapter::mysql::{
		mysql_level_request_repository::MySqlLevelRequestRepository,
		mysql_review_repository::MySqlReviewRepository
	},
	domain::{
		model::{
			api::{
				level_request_api::{
					DiscordID, LevelRequestApiResponseError, PostLevelRequestApiResponse
				},
				level_review_api::{
					GetLevelReviewApiRespnse, LevelReviewApiRequest, LevelReviewApiResponse,
					LevelReviewApiResponseError
				}
			},
			error::level_review_error::LevelReviewError,
			review::LevelReview
		},
		service::{level_review_service::LevelReviewService, review_service::ReviewService}
	}
};

#[get("/review_level/<level_id>")]
pub async fn get_level_review(
	db_conn: &State<DatabaseConnection>,
	level_id: u64,
	discord_id: DiscordID
) -> Result<GetLevelReviewApiRespnse, LevelReviewApiResponseError> {
	let level_review_repository = MySqlReviewRepository::new(db_conn);
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);

	let level_review_service =
		LevelReviewService::new(level_review_repository, level_request_repository);

	match level_review_service
		.get_level_review(level_id, u64::from(discord_id))
		.await
	{
		Ok(level_review) => Ok(GetLevelReviewApiRespnse::from(level_review)),
		Err(get_level_review_error) => Err(get_level_review_error.into())
	}
}

#[post("/review_level", format = "json", data = "<level_review_body>")]
pub async fn review_level<'a>(
	db_conn: &State<DatabaseConnection>,
	level_review_body: Json<LevelReviewApiRequest<'a>>
) -> Result<LevelReviewApiResponse, LevelReviewApiResponseError> {
	let level_review_repository = MySqlReviewRepository::new(db_conn);
	let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
	let level_review_service =
		LevelReviewService::new(level_review_repository, level_request_repository);

	match level_review_service
		.review_level(
			level_review_body.level_id,
			level_review_body.reviewer_discord_id,
			level_review_body.discord_message_id,
			level_review_body.review_contents.to_string()
		)
		.await
	{
		Ok(level_review_info) => Ok(LevelReviewApiResponse {
			level_id: level_review_info.level_id,
			reviewer_discord_id: level_review_info.reviewer_discord_id,
			review_contents: level_review_info.review_contents,
			is_update: level_review_info.is_update
		}),
		Err(level_review_error) => Err(level_review_error.into())
	}
}
