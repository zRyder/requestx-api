use sea_orm::{ActiveValue, IntoActiveModel};

use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			level_request_repository::LevelRequestRepository,
			moderator_repository::ModeratorRepository
		}
	},
	domain::{
		model::{
			error::{
				geometry_dash::geometry_dash_dashrs_error::GeometryDashDashrsError,
				moderator_error::ModeratorError
			},
			level_request::LevelRequest,
			moderator::{Moderator, SuggestedRating, SuggestedScore}
		},
		service::{
			internal::request_manager_service::RequestManagerService,
			moderate_service::ModerateService
		}
	}
};

pub struct ModeratorService<
	'a,
	R: ModeratorRepository,
	L: LevelRequestRepository,
	G: GeometryDashClient
> {
	moderator_repository: &'a R,
	level_request_repository: &'a L,
	gd_client: &'a G,
	request_manager: &'a RequestManagerService
}

impl<'a, R: ModeratorRepository, L: LevelRequestRepository, G: GeometryDashClient> ModerateService
	for ModeratorService<'a, R, L, G>
{
	async fn send_level(
		&self,
		level_id: u64,
		suggested_rating: SuggestedRating,
		suggested_score: SuggestedScore
	) -> Result<(LevelRequest, Moderator), ModeratorError> {
		let mut moderator_data = Moderator {
			level_id,
			suggested_score,
			suggested_rating
		};

		match self
			.level_request_repository
			.get_record(moderator_data.level_id)
			.await
		{
			Ok(Some(level_request)) => {
				if self.request_manager.get_enable_gd_request().await
					&& (moderator_data.suggested_score != SuggestedScore::NoRate
						&& moderator_data.suggested_score != SuggestedScore::Rated)
				{
					if let Err(dashrs_error) = self.gd_client.send_gd_level(moderator_data).await {
						match dashrs_error {
							GeometryDashDashrsError::LevelAlreadyRated(already_rated_level_id) => {
								warn!(
									"Level with ID: {} has already been rated",
									already_rated_level_id
								);
								moderator_data.suggested_score = SuggestedScore::Rated;
								moderator_data.suggested_rating = SuggestedRating::Rate
							}
							_ => {
								error!(
									"Error sending level {:?}: {}",
									moderator_data, dashrs_error
								);
								return Err(ModeratorError::GeometryDashDashrsError);
							}
						}
					}
				}

				match self
					.moderator_repository
					.get_record(moderator_data.level_id)
					.await
				{
					Ok(Some(level_send)) => {
						if moderator_data.suggested_score == SuggestedScore::NoRate {
							error!(
								"Cannot send level with ID {} for no rate",
								moderator_data.level_id
							);
							return Err(ModeratorError::UnsendableLevel);
						}

						let mut previous_level_send = level_send.into_active_model();
						previous_level_send.rating =
							ActiveValue::Set(moderator_data.suggested_rating.into());
						previous_level_send.score =
							ActiveValue::Set(moderator_data.suggested_score.into());

						if let Err(update_error) = self
							.moderator_repository
							.update_record(previous_level_send)
							.await
						{
							error!(
								"Error updating level send record from database: {}",
								update_error
							);
							return Err(ModeratorError::DatabaseError(update_error));
						}
					}
					Ok(None) => {
						if let Err(insert_error) = self
							.moderator_repository
							.create_record(moderator_data.into())
							.await
						{
							error!(
								"Error inserting level send record from database: {}",
								insert_error
							);
							return Err(ModeratorError::DatabaseError(insert_error));
						}
					}
					Err(db_error) => {
						error!("Error reading level send from database: {}", db_error);
						return Err(ModeratorError::DatabaseError(db_error));
					}
				}
				Ok((LevelRequest::from(level_request), moderator_data))
			}
			Ok(None) => {
				warn!("Level request {} does not exist", moderator_data.level_id);
				Err(ModeratorError::LevelRequestDoesNotExists)
			}
			Err(db_error) => {
				error!("Error reading level send from database: {}", db_error);
				Err(ModeratorError::DatabaseError(db_error))
			}
		}
	}
}

impl<'a, R: ModeratorRepository, L: LevelRequestRepository, G: GeometryDashClient>
	ModeratorService<'a, R, L, G>
{
	pub fn new(
		moderator_repository: &'a R,
		level_request_repository: &'a L,
		gd_client: &'a G
	) -> Self {
		ModeratorService {
			moderator_repository,
			level_request_repository,
			gd_client,
			request_manager: &RequestManagerService {}
		}
	}
}
