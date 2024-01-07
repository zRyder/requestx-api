use rocket_framework::serde::json::Json;
use rocket_framework::State;
use sea_orm::DatabaseConnection;
use crate::adapter::mysql::mysql_level_request_repository::MySqlLevelRequestRepository;
use crate::adapter::mysql::mysql_review_repository::MySqlReviewRepository;
use crate::domain::model::api::level_request_api::{LevelRequestApiResponse, LevelRequestApiResponseError};
use crate::domain::model::api::level_review_api::{LevelReviewApiRequest, LevelReviewApiResponse, LevelReviewApiResponseError};
use crate::domain::model::error::level_review_error::LevelReviewError;
use crate::domain::model::review::Review;
use crate::domain::service::level_review_service::LevelReviewService;
use crate::domain::service::review_service::ReviewService;

#[post("/review_level", format = "json", data = "<level_review_body>")]
pub async fn review_level<'a>(
    db_conn: &State<DatabaseConnection>,
    level_review_body: Json<LevelReviewApiRequest<'a>>
) -> Result<LevelReviewApiResponse, LevelReviewApiResponseError> {
    let level_review_repository = MySqlReviewRepository::new(db_conn);
    let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
    let level_review_service = LevelReviewService::new(level_review_repository, level_request_repository);

    match level_review_service.review_level(
        level_review_body.level_id,
        level_review_body.reviewer_discord_id,
        level_review_body.discord_message_id,
        level_review_body.review_contents.to_string()
    ).await {
        Ok(level_review_info) => {
            Ok(LevelReviewApiResponse {
                level_id: level_review_info.level_id,
                reviewer_discord_id: level_review_info.reviewer_discord_id,
                discord_message_id: level_review_info.discord_message_id,
                review_contents: level_review_info.review_contents,
                is_update: level_review_info.is_update
            })
        }
        Err(level_review_error) => {
            Err(level_review_error.into())
        }
    }
}