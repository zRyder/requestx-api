use rocket_framework::futures::future::join_all;
use rocket_framework::{serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::adapter::mysql::moderator_repository::ModeratorRepository;
use crate::domain::model::moderator::{SuggestedRating, SuggestedScore};
use crate::domain::service::moderator_service::ModeratorService;
use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			level_request_repository::LevelRequestRepository, user_repository::UserRepository,
		},
	},
	domain::{
		model::api::{
			auth_api::Auth,
			level_request_api::{
				GetLevelRequestApiResponse, LevelRequestApiResponseError,
				PatchLevelRequestApiRequest, PostLevelRequestApiRequest,
				PostLevelRequestApiResponse,
			},
		},
		service::level_request_service::LevelRequestService,
	},
};

#[get("/request_level/<level_id>")]
pub async fn get_level_request(
	db_conn: &State<DatabaseConnection>,
	level_id: u64,
	_auth: Auth,
) -> Result<GetLevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let user_repository = UserRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();

	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	match level_request_service
		.get_level_request(level_id, None)
		.await
	{
		Ok(level_request_info) => Ok(GetLevelRequestApiResponse::from(level_request_info)),
		Err(get_level_request_error) => Err(get_level_request_error.into()),
	}
}

#[post("/request_level", format = "json", data = "<level_request_body>")]
pub async fn request_level<'a>(
	db_conn: &State<DatabaseConnection>,
	level_request_body: Json<PostLevelRequestApiRequest<'a>>,
	_auth: Auth,
) -> Result<PostLevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let user_repository = UserRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();

	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);
	let request_rating = level_request_body.request_rating.into();
	match level_request_service
		.request_level(
			level_request_body.level_id,
			level_request_body.youtube_video_link.to_string(),
			level_request_body.discord_id,
			request_rating,
			level_request_body.has_requested_feedback,
			level_request_body.notify,
		)
		.await
	{
		Ok(level_request_info) => Ok(PostLevelRequestApiResponse::from(level_request_info)),
		Err(level_request_error) => Err(level_request_error.into()),
	}
}

#[patch(
	"/request_level",
	format = "json",
	data = "<update_level_request_body>"
)]
pub async fn update_level_request<'a>(
	db_conn: &State<DatabaseConnection>,
	update_level_request_body: Json<PatchLevelRequestApiRequest<'a>>,
	_auth: Auth,
) -> Result<GetLevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let user_repository = UserRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();

	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	match level_request_service
		.update_level_request(
			update_level_request_body.level_id,
			update_level_request_body.discord_id,
			update_level_request_body
				.youtube_video_link
				.map(|s| s.to_string()),
			update_level_request_body.request_rating.map(|r| r.into()),
			update_level_request_body.has_requested_feedback,
			update_level_request_body.notify,
		)
		.await
	{
		Ok(level_request_info) => Ok(GetLevelRequestApiResponse::from(level_request_info)),
		Err(level_request_error) => Err(level_request_error.into()),
	}
}

#[delete("/request_level/<level_id>")]
pub async fn delete_level_request<'a>(
	db_conn: &State<DatabaseConnection>,
	level_id: u64,
	_auth: Auth,
) -> Result<GetLevelRequestApiResponse, LevelRequestApiResponseError> {
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let user_repository = UserRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();

	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	match level_request_service.delete_level_request(level_id).await {
		Ok(deleted_level_request) => Ok(GetLevelRequestApiResponse::from(deleted_level_request)),
		Err(delete_level_request_error) => Err(delete_level_request_error.into()),
	}
}

#[get("/request_level/rated")]
pub async fn get_unchecked_level_requests<'a>(
	db_conn: &State<DatabaseConnection>,
	_auth: Auth,
) -> Result<Json<Vec<GetLevelRequestApiResponse>>, LevelRequestApiResponseError> {
	let level_request_repository = LevelRequestRepository::new(db_conn);
	let user_repository = UserRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();
	let moderator_repository = ModeratorRepository::new(db_conn);
	let moderator_service =
		&ModeratorService::new(&moderator_repository, &level_request_repository, &gd_client);

	let level_request_service =
		LevelRequestService::new(&level_request_repository, &user_repository, &gd_client);

	let unchecked_and_unrated_level_requests = level_request_service
		.get_unchecked_and_unrated_level_requests()
		.await
		.map_err(|level_request_error| level_request_error.into())?;

	let level_request_to_send = join_all(unchecked_and_unrated_level_requests.iter().map(
		|item| async move {
			moderator_service
				.send_level(item.level_id, SuggestedRating::Rate, SuggestedScore::Rated)
				.await
		},
	))
	.await;

	let sent_levels = unchecked_and_unrated_level_requests
		.into_iter()
		.zip(level_request_to_send)
		.filter_map(|(level_request, send_level_result)| {
			if send_level_result.is_ok() {
				Some(level_request)
			} else {
				None
			}
		})
		.map(|sent_level| GetLevelRequestApiResponse::from(sent_level))
		.collect::<Vec<GetLevelRequestApiResponse>>();

	Ok(Json(sent_levels))
}
