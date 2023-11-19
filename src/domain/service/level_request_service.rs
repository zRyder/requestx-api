use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::level_request_repository::LevelRequestRepository
	},
	domain::{
		model::{
			error::level_request_error::LevelRequestError, gd_level::GDLevelRequest,
			request_rating::RequestRating
		},
		service::request_service::RequestService
	}
};

pub struct LevelRequestService<R: LevelRequestRepository, G: GeometryDashClient> {
	level_request_repository: R,
	gd_client: G
}

impl<R: LevelRequestRepository, G: GeometryDashClient> RequestService
	for LevelRequestService<R, G>
{
	async fn request<'a>(
		self,
		level_id: u64,
		youtube_video_link: &'a str,
		discord_id: &'a str,
		request_rating: RequestRating
	) -> Result<(), LevelRequestError> {
		let gd_level_result = self.gd_client.get_gd_level_info(level_id).await;

		match gd_level_result {
			Ok(gd_level) => {
				let gd_level_request = GDLevelRequest {
					gd_level,
					request_rating
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
							let level_request_storable = gd_level_request.into();
							if let Err(insert_err) = self
								.level_request_repository
								.create_record(level_request_storable)
								.await
							{
								error!("Unable to save level request for {} to database", level_id);
								Err(LevelRequestError::DatabaseError(insert_err))
							} else {
								Ok(())
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
}

impl<R: LevelRequestRepository, G: GeometryDashClient> LevelRequestService<R, G> {
	pub fn new(level_request_repository: R, gd_client: G) -> Self {
		LevelRequestService {
			level_request_repository,
			gd_client
		}
	}
}

#[cfg(test)]
mod tests {
	use rocket::tokio;
	use sea_orm::InsertResult;
	use tokio_test::assert_ok;

	use crate::{
		adapter::{
			geometry_dash::geometry_dash_client::MockGeometryDashClient,
			mysql::{
				level_request_repository::MockLevelRequestRepository, model::level_request::Model
			}
		},
		domain::{
			model::{
				error::level_request_error::LevelRequestError::LevelRequestExists,
				gd_level::{GDLevel, GDLevelRequest},
				level_creator::LevelCreator,
				request_rating::RequestRating
			},
			service::{
				level_request_service::LevelRequestService, request_service::RequestService
			}
		}
	};

	#[tokio::test]
	async fn test_request_service_should_return_ok() {
		let mut mock_repository = MockLevelRequestRepository::new();
		let mut mock_gd_client = MockGeometryDashClient::new();

		mock_repository
			.expect_get_record()
			.return_once(move |_| Ok(None));
		mock_repository
			.expect_create_record()
			.return_once(move |_| {
				Ok(InsertResult {
					last_insert_id: 99999999
				})
			});

		mock_gd_client
			.expect_get_gd_level_info()
			.return_once(move |_| {
				Ok(GDLevel {
					level_id: 99999999,
					name: "Level Name".to_string(),
					creator: LevelCreator {
						name: "Level Creator".to_string(),
						account_id: 1234,
						player_id: 5678
					},
					description: Some("Level Descritpion".to_string())
				})
			});

		let service = LevelRequestService {
			level_request_repository: mock_repository,
			gd_client: mock_gd_client
		};

		assert_ok!(service.request(99999999, "", "", RequestRating::Easy).await);
	}

	#[tokio::test]
	async fn test_request_service_should_return_error_when_request_already_exists() {
		let mut mock_repository = MockLevelRequestRepository::new();
		let mut mock_gd_client = MockGeometryDashClient::new();

		mock_repository.expect_get_record().return_once(move |_| {
			Ok(Some(Model {
				id: 99999999,
				name: "Level Name".to_string(),
				description: Some("Level Description".to_string()),
				author: "Creator Name".to_string(),
				request_rating: RequestRating::Easy.into()
			}))
		});

		mock_gd_client
			.expect_get_gd_level_info()
			.return_once(move |_| {
				Ok(GDLevel {
					level_id: 99999999,
					name: "Level Name".to_string(),
					creator: LevelCreator {
						name: "Level Creator".to_string(),
						account_id: 1234,
						player_id: 5678
					},
					description: Some("Level Descritpion".to_string())
				})
			});

		let service = LevelRequestService {
			level_request_repository: mock_repository,
			gd_client: mock_gd_client
		};

		assert_eq!(
			service.request(99999999, "", "", RequestRating::Easy).await,
			Err(LevelRequestExists)
		)
	}

	fn setup_level_helper(
		level_id: u64,
		name: String,
		creator_name: String,
		account_id: u64,
		player_id: u64,
		description: Option<String>,
		request_rating: RequestRating
	) -> GDLevelRequest {
		GDLevelRequest {
			gd_level: GDLevel {
				level_id,
				name,
				creator: LevelCreator {
					name: creator_name,
					account_id,
					player_id
				},
				description
			},
			request_rating
		}
	}
}
