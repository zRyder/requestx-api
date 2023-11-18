use crate::{
	adapter::mysql::level_request_repository::LevelRequestRepository,
	domain::{
		model::{error::level_request_error::LevelRequestError, gd_level::GDLevelRequest},
		service::request_service::RequestService
	}
};

pub struct LevelRequestService<R: LevelRequestRepository> {
	pub level: GDLevelRequest,
	pub repository: R
}

impl<R: LevelRequestRepository> RequestService for LevelRequestService<R> {
	async fn request(self) -> Result<(), LevelRequestError> {
		let request_level_result = self.repository.get_record(self.level.level.level_id).await;
		match request_level_result {
			Ok(some_request) => {
				if some_request.is_some() {
					Err(LevelRequestError::LevelRequestExists)
				} else {
					let level_request_storable = self.level.into();
					if let Err(insert_err) =
						self.repository.create_record(level_request_storable).await
					{
						Err(LevelRequestError::DatabaseError(insert_err))
					} else {
						Ok(())
					}
				}
			}
			Err(err) => Err(LevelRequestError::DatabaseError(err))
		}
	}
}

#[cfg(test)]
mod tests {
	use rocket::tokio;
	use sea_orm::InsertResult;
	use tokio_test::assert_ok;

	use crate::{
		adapter::mysql::{
			level_request_repository::MockLevelRequestRepository, model::level_request::Model
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
		let gd_level = setup_level_helper(
			99999999,
			"Level Name".to_string(),
			"Creator Name".to_string(),
			1234,
			5678,
			Some("Level Description".to_string()),
			RequestRating::Easy
		);

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

		let service = LevelRequestService {
			level: gd_level,
			repository: mock_repository
		};

		assert_ok!(service.request().await);
	}

	#[tokio::test]
	async fn test_request_service_should_return_error_when_request_already_exists() {
		let mut mock_repository = MockLevelRequestRepository::new();
		let gd_level = setup_level_helper(
			99999999,
			"Level Name".to_string(),
			"Creator Name".to_string(),
			1234,
			5678,
			Some("Level Description".to_string()),
			RequestRating::Easy
		);

		mock_repository.expect_get_record().return_once(move |_| {
			Ok(Some(Model {
				id: 99999999,
				name: "Level Name".to_string(),
				description: Some("Level Description".to_string()),
				author: "Creator Name".to_string(),
				request_rating: RequestRating::Easy.into()
			}))
		});

		let service = LevelRequestService {
			level: gd_level,
			repository: mock_repository
		};

		assert_eq!(service.request().await, Err(LevelRequestExists))
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
			level: GDLevel {
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
