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
			error::moderator_error::ModeratorError,
			gd_level::GDLevelRequest,
			moderator::{Moderator, SuggestedRating, SuggestedScore}
		},
		service::moderate_service::ModerateService
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
	gd_client: &'a G
}

impl<'a, R: ModeratorRepository, L: LevelRequestRepository, G: GeometryDashClient> ModerateService
	for ModeratorService<'a, R, L, G>
{
	async fn send_level(
		&self,
		level_id: u64,
		suggested_rating: SuggestedRating,
		suggested_score: SuggestedScore
	) -> Result<GDLevelRequest, ModeratorError> {
		let moderator_data = Moderator {
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

				if moderator_data.suggested_score != SuggestedScore::NoRate {
					if let Err(dashrs_error) = self.gd_client.send_gd_level(moderator_data).await {
						error!("Error sending level {:?}: {}", moderator_data, dashrs_error);
						return Err(ModeratorError::GeometryDashDashrsError);
					}
				}
				Ok(GDLevelRequest::from(level_request))
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
			gd_client
		}
	}
}
