use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			level_request_repository::LevelRequestRepository, user_repository::UserRepository,
		},
	},
	domain::{
		model::{
			discord::{message::DiscordMessage, user::DiscordUser},
			error::level_request_error::LevelRequestError,
			level_request::{GDLevel, LevelCreator, LevelRequest, RequestRating},
		},
		service::internal::request_manager_service::RequestManagerService,
	},
	rocket::common::{config::common_config::APP_CONFIG, constants::YOUTUBE_LINK_REGEX},
};
use chrono::{DateTime, Duration, Utc};
use std::cmp::PartialEq;

pub struct LevelRequestService<'a> {
	level_request_repository: &'a LevelRequestRepository<'a>,
	user_repository: &'a UserRepository<'a>,
	gd_client: &'a GeometryDashClient,
	request_manager: &'a RequestManagerService,
}

impl<'a> LevelRequestService<'a> {
	pub fn new(
		level_request_repository: &'a LevelRequestRepository,
		user_repository: &'a UserRepository,
		gd_client: &'a GeometryDashClient,
	) -> Self {
		LevelRequestService {
			level_request_repository,
			user_repository,
			gd_client,
			request_manager: &RequestManagerService {},
		}
	}

	pub async fn get_level_request(
		&self,
		level_id: u64,
		has_requested_feedback: Option<bool>,
	) -> Result<LevelRequest, LevelRequestError> {
		let get_level_request_result =
			if let Some(has_requested_feedback_toggle) = has_requested_feedback {
				self.level_request_repository
					.get_record_filter_feedback(level_id, has_requested_feedback_toggle)
					.await
			} else {
				self.level_request_repository.get_record(level_id).await
			};

		get_level_request_result
			.map_err(|query_level_request_error| {
				error!(
					"Error fetching level request record: {}",
					query_level_request_error
				);
				LevelRequestError::DatabaseError(query_level_request_error)
			})?
			.map_or_else(
				|| {
					warn!("Level request with ID {} does not exist", level_id);
					Err(LevelRequestError::LevelRequestDoesNotExist)
				},
				|level_request_record| Ok(LevelRequest::from(level_request_record)),
			)
	}

	pub async fn request_level(
		&self,
		level_id: u64,
		youtube_video_link: String,
		discord_user_id: u64,
		request_rating: RequestRating,
		has_requested_feedback: bool,
		notify: bool,
	) -> Result<LevelRequest, LevelRequestError> {
		if let Some(validate_level_request_error) = self
			.validate_level_request(level_id, &youtube_video_link)
			.await
		{
			return Err(validate_level_request_error);
		}

		let now = Utc::now();
		let is_gd_requests_enabled = self.request_manager.get_enable_gd_request().await;

		let level_request = if is_gd_requests_enabled {
			let gd_level = self.gd_client.get_gd_level_info(level_id).await.map_err(
				|get_gd_level_info_error| {
					error!("Error getting level info for level {}", level_id);
					LevelRequestError::GeometryDashClientError(level_id, get_gd_level_info_error)
				},
			)?;
			LevelRequest::with_gd_level(
				gd_level,
				level_id,
				discord_user_id,
				request_rating,
				youtube_video_link,
				has_requested_feedback,
				notify,
				now,
			)
		} else {
			LevelRequest::new(
				level_id,
				discord_user_id,
				request_rating,
				youtube_video_link,
				has_requested_feedback,
				notify,
				now,
			)
		};

		let mut discord_user = self.get_user(discord_user_id).await?;
		if let Some(level_request_error) = self
			.check_user_can_request_level(&discord_user, &level_request, &now)
			.await
		{
			return Err(level_request_error);
		};
		discord_user.last_request_time = Some(now);

		let discord_user_storable = discord_user.into();
		if let Err(create_or_update_discord_user_error) = self
			.user_repository
			.create_or_update_record(discord_user_storable)
			.await
		{
			error!(
				"Error creating or updating user record: {}",
				discord_user_id
			);
			return Err(LevelRequestError::DatabaseError(
				create_or_update_discord_user_error,
			));
		}

		let level_request_storable = level_request.clone().into();
		if let Err(level_insert_error) = self
			.level_request_repository
			.create_record(level_request_storable)
			.await
		{
			error!(
				"Unable to save level request for {} to database: {}",
				level_id, level_insert_error
			);
			return Err(LevelRequestError::DatabaseError(level_insert_error));
		}

		Ok(level_request)
	}

	async fn check_user_can_request_level(
		&self,
		discord_user: &DiscordUser,
		level_request: &LevelRequest,
		now: &DateTime<Utc>,
	) -> Option<LevelRequestError> {
		let gd_http_requests_enabled = self.request_manager.get_enable_gd_request().await;
		let cooldown_duration = self.request_manager.get_request_cooldown().await;
		let allow_platformer_levels = self.request_manager.get_allow_platformer_levels().await;
		let allow_non_user_created_levels = self
			.request_manager
			.get_allow_non_user_created_levels()
			.await;

		if Self::is_user_on_cooldown(&discord_user, now, &cooldown_duration) {
			warn!(
				"User {} attempted to request while on cooldown",
				discord_user.discord_user_id
			);
			return Some(LevelRequestError::UserOnCooldown(*now, cooldown_duration));
		};

		if gd_http_requests_enabled {
			if !allow_non_user_created_levels
				&& !Self::is_user_created_level_request(discord_user, level_request)
			{
				warn!(
					"User {} attempted to request a level they did not create",
					discord_user.discord_user_id
				);

				return Some(LevelRequestError::RequestNonCreatedLevel);
			};

			if !allow_platformer_levels && level_request.is_platformer_level() {
				warn!(
					"User {} attempted to request a platformer level when disabled",
					discord_user.discord_user_id
				);

				return Some(LevelRequestError::RequestPlatformer);
			}
		}

		None
	}

	async fn validate_level_request(
		&self,
		level_id: u64,
		youtube_video_link: &String,
	) -> Option<LevelRequestError> {
		if !self.request_manager.get_enable_request().await {
			return Some(LevelRequestError::LevelRequestsDisabled);
		}
		if !Self::is_valid_youtube_link(&youtube_video_link) {
			warn!("Malformed YouTube link: {}", youtube_video_link);
			return Some(LevelRequestError::MalformedRequest);
		}
		if let Ok(_existing_level_request) = self.get_level_request(level_id, None).await {
			warn!("Level requests with ID: {} already exists", level_id);
			return Some(LevelRequestError::LevelRequestExists);
		}
		None
	}

	pub async fn update_level_request(
		&self,
		level_id: u64,
		discord_user_id: u64,
		youtube_video_link: Option<String>,
		request_rating: Option<RequestRating>,
		has_requested_feedback: Option<bool>,
		notify: Option<bool>,
	) -> Result<LevelRequest, LevelRequestError> {
		if youtube_video_link.is_none()
			&& request_rating.is_none()
			&& has_requested_feedback.is_none()
			&& notify.is_none()
		{
			warn!("No edited data");
			return Err(LevelRequestError::MalformedRequest);
		}
		if youtube_video_link.is_some()
			&& !Self::is_valid_youtube_link(&youtube_video_link.as_ref().unwrap())
		{
			warn!("Malformed YouTube link: {}", youtube_video_link.unwrap());
			return Err(LevelRequestError::MalformedRequest);
		}
		let mut existing_level_request = self
			.level_request_repository
			.get_record(level_id)
			.await
			.map_err(|get_existing_level_request_error| {
				error!(
					"Error getting existing level request: {}",
					get_existing_level_request_error
				);
				LevelRequestError::DatabaseError(get_existing_level_request_error)
			})?
			.map(LevelRequest::from)
			.map(Ok)
			.unwrap_or_else(|| {
				warn!("Level request with id {} does not exist", level_id);
				Err(LevelRequestError::LevelRequestDoesNotExist)
			})?;

		if !discord_user_id.eq(&APP_CONFIG.get().unwrap().server_config.discord_bot_admin_id)
			&& !discord_user_id.eq(&existing_level_request.discord_user_id)
		{
			error!(
				"User {} attempted to edit a level request {} they do not own",
				discord_user_id, level_id
			);
			return Err(LevelRequestError::EditUnownedLevelRequest(
				existing_level_request.level_id,
				existing_level_request.discord_user_id,
				discord_user_id,
			));
		}

		let is_gd_requests_enabled = self.request_manager.get_enable_gd_request().await;

		self.update_level_request_params(
			level_id,
			youtube_video_link,
			request_rating,
			has_requested_feedback,
			notify,
			is_gd_requests_enabled,
			&mut existing_level_request,
		)
		.await?;

		self.level_request_repository
			.update_record(existing_level_request.into())
			.await
			.map(LevelRequest::from)
			.map_err(|update_level_request_error| {
				error!(
					"Unable to update level request for {} to database: {}",
					level_id, update_level_request_error
				);
				LevelRequestError::DatabaseError(update_level_request_error)
			})
	}

	pub async fn delete_level_request(
		&self,
		level_id: u64,
	) -> Result<LevelRequest, LevelRequestError> {
		let existing_level_request = self
			.level_request_repository
			.get_record(level_id)
			.await
			.map_err(|get_existing_level_request_error| {
				error!(
					"Error getting existing level request from database: {}",
					get_existing_level_request_error
				);
				LevelRequestError::DatabaseError(get_existing_level_request_error)
			})?
			.map(LevelRequest::from)
			.map(Ok)
			.unwrap_or_else(|| {
				error!("Level request {} does not exist", level_id);
				Err(LevelRequestError::LevelRequestDoesNotExist)
			})?;

		if let Err(delete_level_request_error) = self
			.level_request_repository
			.delete_record(existing_level_request.clone().into())
			.await
		{
			error!(
				"Unable to delete level request from database: {}",
				delete_level_request_error
			);
			return Err(LevelRequestError::DatabaseError(delete_level_request_error));
		}

		Ok(existing_level_request)
	}

	pub async fn get_unchecked_and_unrated_level_requests(
		&self,
	) -> Result<Vec<LevelRequest>, LevelRequestError> {
		let unchecked_levels_request_list = self
			.level_request_repository
			.get_all_unchecked_records()
			.await
			.map_err(|get_unchecked_levels_request_list_error| {
				error!(
					"Error getting existing level request from database: {}",
					get_unchecked_levels_request_list_error
				);

				LevelRequestError::DatabaseError(get_unchecked_levels_request_list_error)
			})?
			.into_iter()
			.map(|unchecked_levels_request_record| {
				LevelRequest::from(unchecked_levels_request_record.0)
			})
			.collect::<Vec<LevelRequest>>();

		let mut unchecked_and_rated_level_requests_list: Vec<LevelRequest> = Vec::new();
		for unchecked_level_request in unchecked_levels_request_list {
			let Some(is_rated) = self
				.gd_client
				.is_rated(unchecked_level_request.level_id)
				.await
				.map_err(|gd_client_error| {
					error!(
						"Error checking if level with ID {} is rated {}",
						unchecked_level_request.level_id, gd_client_error
					);
				})
				.ok()
			else {
				continue;
			};

			if is_rated {
				unchecked_and_rated_level_requests_list.push(unchecked_level_request);
			}
		}

		Ok(unchecked_and_rated_level_requests_list)
	}

	pub async fn update_level_request_message_id(
		&self,
		level_id: u64,
		discord_message_id: u64,
	) -> Result<(), LevelRequestError> {
		let mut existing_level_request = self
			.level_request_repository
			.get_record(level_id)
			.await
			.map_err(|get_existing_level_request_error| {
				error!(
					"Error getting existing level request from database: {}",
					get_existing_level_request_error
				);
				LevelRequestError::DatabaseError(get_existing_level_request_error)
			})?
			.map(LevelRequest::from)
			.map(Ok)
			.unwrap_or_else(|| {
				error!("Level request {} does not exist", level_id);
				Err(LevelRequestError::LevelRequestDoesNotExist)
			})?;

		existing_level_request.discord_message_data = Some(DiscordMessage {
			message_id: discord_message_id,
		});
		if let Err(update_level_request_record) = self
			.level_request_repository
			.update_record(existing_level_request.into())
			.await
		{
			error!(
				"Error updating level request with level ID: {}: {}",
				level_id, update_level_request_record
			);
			return Err(LevelRequestError::DatabaseError(
				update_level_request_record,
			));
		}

		Ok(())
	}

	async fn update_level_request_params(
		&self,
		level_id: u64,
		youtube_video_link: Option<String>,
		request_rating: Option<RequestRating>,
		has_requested_feedback: Option<bool>,
		notify: Option<bool>,
		is_gd_requests_enabled: bool,
		level_request: &mut LevelRequest,
	) -> Result<(), LevelRequestError> {
		if let Some(youtube_video_link) = youtube_video_link {
			level_request.youtube_video_link = youtube_video_link
		}
		if let Some(request_rating) = request_rating {
			level_request.request_rating = request_rating
		}
		if let Some(has_requested_feedback) = has_requested_feedback {
			level_request.has_requested_feedback = has_requested_feedback
		}
		if let Some(notify) = notify {
			level_request.notify = notify
		}

		if is_gd_requests_enabled {
			let gd_level = self
				.gd_client
				.get_gd_level_info(level_id)
				.await
				.map_err(|err| {
					error!("Error getting level info for level {}", level_id);
					LevelRequestError::GeometryDashClientError(level_id, err)
				})?;
			let gd_level_to_update = GDLevel {
				name: gd_level.name,
				creator: LevelCreator {
					name: gd_level.creator.name,
					player_id: gd_level.creator.player_id,
				},
				level_length: gd_level.level_length,
			};

			level_request.gd_level = Some(gd_level_to_update);
		}

		Ok(())
	}

	async fn get_user<'b>(&self, discord_user_id: u64) -> Result<DiscordUser, LevelRequestError> {
		Ok(self
			.user_repository
			.get_record(discord_user_id)
			.await
			.map_err(|get_user_record_error| {
				error!(
					"Error getting Discord user: {} record from database: {}",
					discord_user_id, get_user_record_error
				);
				LevelRequestError::DatabaseError(get_user_record_error)
			})?
			.map(DiscordUser::from)
			.unwrap_or_else(|| DiscordUser::new(discord_user_id)))
	}

	fn is_valid_youtube_link(youtube_link: &str) -> bool {
		let regex = regex::RegexBuilder::new(YOUTUBE_LINK_REGEX)
			.case_insensitive(true)
			.multi_line(true)
			.build()
			.unwrap();

		regex.is_match(youtube_link)
	}

	fn is_user_on_cooldown(
		discord_user: &DiscordUser,
		now: &DateTime<Utc>,
		cooldown_duration: &Duration,
	) -> bool {
		if let Some(discord_user_last_request_time) = discord_user.last_request_time {
			(discord_user_last_request_time + *cooldown_duration).ge(now)
		} else {
			false
		}
	}

	fn is_user_created_level_request(
		discord_user: &DiscordUser,
		level_request: &LevelRequest,
	) -> bool {
		match (discord_user.gd_player_id, level_request.gd_level.as_ref()) {
			(Some(discord_user_gd_player_id), Some(gd_level)) => {
				discord_user_gd_player_id == gd_level.creator.player_id
			}
			_ => false,
		}
	}
}

// #[cfg(test)]
// mod tests {
// 	use rocket::tokio;
// 	use sea_orm::InsertResult;
// 	use tokio_test::assert_ok;
//
// 	use crate::{
// 		adapter::{
// 			geometry_dash::geometry_dash_client::MockGeometryDashClient,
// 			mysql::{
// 				level_request_repository::MockLevelRequestRepository,
// model::level_request::Model, 				user_repository::MockUserRepository
// 			}
// 		},
// 		domain::{
// 			model::{
// 				discord::{message::DiscordMessage, user::DiscordUser},
// 				error::level_request_error::LevelRequestError::LevelRequestExists,
// 				gd_level::{GDLevel, GDLevelRequest},
// 				level_creator::LevelCreator,
// 				request_rating::RequestRating
// 			},
// 			service::{
// 				level_request_service::LevelRequestService,
// request_service::RequestService 			}
// 		}
// 	};
//
// 	#[tokio::test]
// 	async fn test_request_service_should_return_ok() {
// 		let mut mock_level_request_repository = MockLevelRequestRepository::new();
// 		let mut mock_user_repository = MockUserRepository::new();
// 		let mut mock_gd_client = MockGeometryDashClient::new();
//
// 		mock_level_request_repository
// 			.expect_get_record()
// 			.return_once(move |_| Ok(None));
// 		mock_level_request_repository
// 			.expect_create_record()
// 			.return_once(move |_| {
// 				Ok(InsertResult {
// 					last_insert_id: 99999999
// 				})
// 			});
//
// 		mock_gd_client
// 			.expect_get_gd_level_info()
// 			.return_once(move |_| {
// 				Ok(GDLevel {
// 					level_id: 99999999,
// 					name: "Level Name".to_string(),
// 					creator: LevelCreator {
// 						name: "Level Creator".to_string(),
// 						account_id: 1234,
// 						player_id: 5678
// 					},
// 					description: Some("Level Descritpion".to_string())
// 				})
// 			});
//
// 		let service = LevelRequestService {
// 			level_request_repository: mock_level_request_repository,
// 			user_repository: mock_user_repository,
// 			gd_client: mock_gd_client
// 		};
//
// 		assert_ok!(
// 			service
// 				.make_level_request(99999999, "LINK".to_string(), 99999999,
// RequestRating::Two) 				.await
// 		);
// 	}
//
// 	#[tokio::test]
// 	async fn test_request_service_should_return_error_when_request_already_exists() {
// 		let mut mock_level_request_repository = MockLevelRequestRepository::new();
// 		let mut mock_user_repository = MockUserRepository::new();
// 		let mut mock_gd_client = MockGeometryDashClient::new();
//
// 		mock_level_request_repository
// 			.expect_get_record()
// 			.return_once(move |_| {
// 				Ok(Some(Model {
// 					level_id: 99999999,
// 					discord_id: 99999999,
// 					discord_message_id: Some(476936521364123),
// 					discord_thread_id: None,
// 					name: "Level Name".to_string(),
// 					description: Some("Level Description".to_string()),
// 					author: "Creator Name".to_string(),
// 					request_rating: RequestRating::Two.into(),
// 					you_tube_video_link: "LINK".to_string()
// 				}))
// 			});
//
// 		mock_gd_client
// 			.expect_get_gd_level_info()
// 			.return_once(move |_| {
// 				Ok(GDLevel {
// 					level_id: 99999999,
// 					name: "Level Name".to_string(),
// 					creator: LevelCreator {
// 						name: "Level Creator".to_string(),
// 						account_id: 1234,
// 						player_id: 5678
// 					},
// 					description: Some("Level Descritpion".to_string())
// 				})
// 			});
//
// 		let service = LevelRequestService {
// 			level_request_repository: mock_level_request_repository,
// 			user_repository: mock_user_repository,
// 			gd_client: mock_gd_client
// 		};
//
// 		assert_eq!(
// 			service
// 				.make_level_request(99999999, "LINK".to_string(), 99999999,
// RequestRating::Two) 				.await,
// 			Err(LevelRequestExists)
// 		)
// 	}
//
// 	fn setup_level_helper(
// 		level_id: u64,
// 		discord_id: u64,
// 		discord_message_id: u64,
// 		name: String,
// 		creator_name: String,
// 		account_id: u64,
// 		player_id: u64,
// 		description: Option<String>,
// 		request_rating: RequestRating,
// 		youtube_video_link: String
// 	) -> GDLevelRequest {
// 		GDLevelRequest {
// 			gd_level: GDLevel {
// 				level_id,
// 				name,
// 				creator: LevelCreator {
// 					name: creator_name,
// 					account_id,
// 					player_id
// 				},
// 				description
// 			},
// 			discord_user_data: DiscordUser { discord_user_id: discord_id },
// 			discord_message_data: Some(DiscordMessage {
// 				message_id: discord_message_id,
// 				thread_id: None,
// 			}),
// 			request_rating,
// 			youtube_video_link
// 		}
// 	}
// }
