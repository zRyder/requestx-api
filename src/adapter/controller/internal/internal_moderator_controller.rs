use rocket_framework::serde::json::Json;
use rocket_framework::State;
use sea_orm::DatabaseConnection;
use crate::adapter::geometry_dash::geometry_dash_dashrs_client::GeometryDashDashrsClient;
use crate::adapter::mysql::my_sql_moderator_repository::MySqlModeratorRepository;
use crate::adapter::mysql::mysql_level_request_repository::MySqlLevelRequestRepository;
use crate::domain::model::api::auth_api::Auth;
use crate::domain::model::api::level_request_api::{GetLevelRequestApiResponse};
use crate::domain::model::internal::api::moderator_api::{ModeratorApiResponseError, PostModeratorApiRequest};
use crate::domain::service::moderate_service::ModerateService;
use crate::domain::service::moderator_service::ModeratorService;

#[post("/send_level", format = "json", data="<send_level_body>")]
pub async fn send_level<'a>(
    db_conn: &State<DatabaseConnection>,
    send_level_body: Json<PostModeratorApiRequest>,
    _auth: Auth
) -> Result<GetLevelRequestApiResponse, ModeratorApiResponseError> {
    let level_request_repository = MySqlLevelRequestRepository::new(db_conn);
    let moderator_repository = MySqlModeratorRepository::new(db_conn);
    let gd_client = GeometryDashDashrsClient::new();
    let moderator_service = ModeratorService::new(moderator_repository, level_request_repository, gd_client);

    match moderator_service.send_level(
        send_level_body.level_id,
        send_level_body.suggested_rating.into(),
        send_level_body.suggested_score.into()
    ).await {
        Ok(level_request_data) => {Ok(GetLevelRequestApiResponse::from(level_request_data))}
        Err(send_level_error) => {Err(send_level_error.into())}
    }
}