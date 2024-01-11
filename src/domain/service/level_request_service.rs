use sea_orm::{ActiveValue::Set, IntoActiveModel};

use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			level_request_repository::LevelRequestRepository, model::level_request::ActiveModel,
			user_repository::UserRepository
		}
	},
	domain::{
		model::{
			discord::user::DiscordUser, error::level_request_error::LevelRequestError,
			gd_level::GDLevelRequest, request_rating::RequestRating
		},
		service::request_service::RequestService
	}
};

pub struct LevelRequestService<L: LevelRequestRepository, U: UserRepository, G: GeometryDashClient>
{
	level_request_repository: L,
	user_repository: U,
	gd_client: G
}

impl<R: LevelRequestRepository, U: UserRepository, G: GeometryDashClient> RequestService
	for LevelRequestService<R, U, G>
{
	async fn get_level_request(
		self,
		level_id: u64,
	) -> Result<GDLevelRequest, LevelRequestError> {
		match self.level_request_repository.get_record(level_id).await {
			Ok(potential_level_request) => {
				if let Some(level_request_model) = potential_level_request {
					let gd_level_request = GDLevelRequest::from(level_request_model);
					Ok(gd_level_request)
				} else {
					warn!("Level request with ID {} does not exist", level_id);
					Err(LevelRequestError::LevelRequestDoesNotExist)
				}
			}
			Err(db_err) => {
				error!(
					"Error making get level request for level {} record database: {}",
					level_id, db_err
				);
				Err(LevelRequestError::DatabaseError(db_err))
			}
		}
	}

	async fn make_level_request(
		self,
		level_id: u64,
		youtube_video_link: String,
		discord_user_id: u64,
		request_rating: RequestRating
	) -> Result<GDLevelRequest, LevelRequestError> {
		match self.gd_client.get_gd_level_info(level_id).await {
			Ok(gd_level) => {
				let gd_level_request = GDLevelRequest {
					gd_level,
					discord_user_data: DiscordUser { discord_user_id },
					discord_message_data: None,
					request_rating,
					youtube_video_link
				};

				let request_level_result = self
					.level_request_repository
					.get_record(gd_level_request.gd_level.level_id)
					.await;
				match request_level_result {
					Ok(some_request) => {
						if some_request.is_some() {
							Err(LevelRequestError::LevelRequestExists)
						} else {
							match self
								.user_repository
								.get_record(gd_level_request.discord_user_data.discord_user_id)
								.await
							{
								Ok(potential_user) => {
									if potential_user.is_none() {
										let user_storable =
											gd_level_request.discord_user_data.into();
										if let Err(user_insert_error) =
											self.user_repository.create_record(user_storable).await
										{
											error!(
												"Unable to save Discord user {} to database: {}",
												gd_level_request.discord_user_data.discord_user_id,
												user_insert_error
											);
											return Err(LevelRequestError::DatabaseError(
												user_insert_error
											));
										}
									}
									let level_request_storable = gd_level_request.clone().into();
									if let Err(level_insert_error) = self
										.level_request_repository
										.create_record(level_request_storable)
										.await
									{
										error!(
											"Unable to save level request for {} to database: {}",
											level_id, level_insert_error
										);
										Err(LevelRequestError::DatabaseError(level_insert_error))
									} else {
										Ok(gd_level_request)
									}
								}
								Err(err) => {
									error!(
										"Error getting Discord user: {} record from database: {}",
										gd_level_request.discord_user_data.discord_user_id, err
									);
									Err(LevelRequestError::DatabaseError(err))
								}
							}
						}
					}
					Err(err) => {
						error!(
							"Error making get level request for level {} record database: {}",
							level_id, err
						);
						Err(LevelRequestError::DatabaseError(err))
					}
				}
			}
			Err(dashrs_err) => {
				error!("Error getting level info for level {}", level_id);
				Err(LevelRequestError::GeometryDashClientError(
					level_id, dashrs_err
				))
			}
		}
	}

	async fn update_level_request_message_id(
		self,
		level_id: u64,
		discord_message_id: u64
	) -> Result<(), LevelRequestError> {
		match self.level_request_repository.get_record(level_id).await {
			Ok(potential_level_request) => {
				if let Some(level_request) = potential_level_request {
					let mut update_level_request_storable: ActiveModel =
						level_request.into_active_model();
					update_level_request_storable.discord_message_id =
						Set(Some(discord_message_id));
					match self
						.level_request_repository
						.update_record(update_level_request_storable)
						.await
					{
						Ok(_updated_record) => Ok(()),
						Err(db_err) => {
							error!(
								"Error updating level request with level ID: {}: {}",
								level_id, db_err
							);
							Err(LevelRequestError::DatabaseError(db_err))
						}
					}
				} else {
					Err(LevelRequestError::LevelRequestDoesNotExist)
				}
			}
			Err(db_err) => {
				error!("Error getting level info for level {}", level_id);
				Err(LevelRequestError::DatabaseError(db_err))
			}
		}
	}

	async fn update_level_request_thread_id(
		self,
		level_id: u64,
		discord_thread_id: u64
	) -> Result<(), LevelRequestError> {
		match self.level_request_repository.get_record(level_id).await {
			Ok(potential_level_request) => {
				if let Some(level_request) = potential_level_request {
					let mut update_level_request_storable: ActiveModel =
						level_request.into_active_model();
					update_level_request_storable.discord_thread_id = Set(Some(discord_thread_id));
					match self
						.level_request_repository
						.update_record(update_level_request_storable)
						.await
					{
						Ok(_updated_record) => Ok(()),
						Err(db_err) => {
							error!(
								"Error updating level request with level ID: {}: {}",
								level_id, db_err
							);
							Err(LevelRequestError::DatabaseError(db_err))
						}
					}
				} else {
					Err(LevelRequestError::LevelRequestDoesNotExist)
				}
			}
			Err(db_err) => {
				error!("Error getting level info for level {}", level_id);
				Err(LevelRequestError::DatabaseError(db_err))
			}
		}
	}
}

impl<R: LevelRequestRepository, U: UserRepository, G: GeometryDashClient>
	LevelRequestService<R, U, G>
{
	pub fn new(level_request_repository: R, user_repository: U, gd_client: G) -> Self {
		LevelRequestService {
			level_request_repository,
			user_repository,
			gd_client
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
// 				level_request_service::LevelRequestService, request_service::RequestService
// 			}
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
