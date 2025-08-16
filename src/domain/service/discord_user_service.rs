use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			gd_account_link_repository::GDAccountLinkRepository, user_repository::UserRepository
		}
	},
	domain::{
		model::{
			discord::user::{DiscordGDAccountLink, DiscordUser, GDAccountLink},
			error::{
				discord::discord_error::DiscordError,
				geometry_dash::geometry_dash_dashrs_error::GeometryDashDashrsError
			}
		},
		service::user_service::UserService
	}
};

pub struct DiscordUserService<'a, U: UserRepository, G: GeometryDashClient> {
	user_repository: &'a U,
	gd_account_link_repository: &'a GDAccountLinkRepository<'a>,
	geometry_dash_client: &'a G
}

impl<'a, U: UserRepository, G: GeometryDashClient> UserService for DiscordUserService<'a, U, G> {
	async fn get_user(&self, discord_user_id: u64) -> Result<DiscordUser, DiscordError> {
		self.user_repository
			.get_record(discord_user_id)
			.await
			.map_err(|query_user_error| {
				error!(
					"Error getting user record from database: {}",
					query_user_error
				);
				DiscordError::DatabaseError(query_user_error)
			})?
			.map_or_else(
				|| {
					warn!("Discord user with ID {} does not exist", discord_user_id);
					Err(DiscordError::UserDoesNotExist)
				},
				|user_record| Ok(DiscordUser::from(user_record))
			)
	}

	async fn init_gd_account_link(
		&self,
		discord_user_id: u64,
		gd_username: String
	) -> Result<DiscordGDAccountLink, DiscordError> {
		let discord_user = match self.user_repository.get_record(discord_user_id).await {
			Ok(Some(user_record)) => DiscordUser::from(user_record),
			Ok(None) => {
				warn!("Discord user with ID {} does not exist", discord_user_id);
				let new_discord_user = DiscordUser::new(discord_user_id);
				if let Err(create_user_record_error) = self
					.user_repository
					.create_record(new_discord_user.clone().into())
					.await
				{
					error!(
						"Error creating Discord user record: {}",
						create_user_record_error
					);
					return Err(DiscordError::DatabaseError(create_user_record_error));
				};
				new_discord_user
			}
			Err(query_discord_user_error) => {
				error!(
					"Error getting user record from database: {}",
					query_discord_user_error
				);
				return Err(DiscordError::DatabaseError(query_discord_user_error));
			}
		};

		if discord_user.gd_player_id.is_some() {
			warn!(
				"Discord account with id {} is already linked to a gd account: {}",
				discord_user.discord_user_id,
				discord_user.gd_player_id.unwrap()
			);
			return Err(DiscordError::DiscordAccountAlreadyLinked);
		}

		let gd_player_id = self
			.geometry_dash_client
			.query_gd_player_and_account_id(&gd_username)
			.await
			.map_err(|query_gd_player_id_error| {
				if let GeometryDashDashrsError::UserNotFoundError(_) = query_gd_player_id_error {
					warn!(
						"Geometry Dash user with username {} does not exist",
						&gd_username
					);
					DiscordError::GDAccountDoesNotExist(gd_username.clone())
				} else {
					error!(
						"Error querying gd player_id from Geometry Dash servers: {}",
						query_gd_player_id_error
					);
					DiscordError::DiscordError
				}
			})?
			.0;

		// if let Some(_existing_gd_account_link) = self
		// 	.gd_account_link_repository
		// 	.get_record(discord_user.discord_user_id)
		// 	.await
		// 	.map_err(|query_gd_account_link_error| {
		// 		error!(
		// 			"Error querying GD account link from database: {}",
		// 			query_gd_account_link_error
		// 		);
		// 		DiscordError::DatabaseError(query_gd_account_link_error)
		// 	})? {
		// 	error!("GD Account has already been linked to another Discord user");
		// 	return Err(DiscordError::DiscordAccountAlreadyLinked);
		// };

		let mut gd_account_link = GDAccountLink::new(discord_user.discord_user_id, gd_player_id);
		gd_account_link.generate_new_account_link();
		let gd_account_challenge = gd_account_link.gd_account_challenge.clone();

		// let updated_user = self
		// 	.user_repository
		// 	.update_record(discord_user.into())
		// 	.await
		// 	.map_err(|update_user_record_error| {
		// 		error!("Error updating user record: {}", update_user_record_error);
		// 		DiscordError::DatabaseError(update_user_record_error)
		// 	})
		// 	.map(|updated_user_record| DiscordUser::from(updated_user_record))?;

		self.gd_account_link_repository
			.create_or_update_record(gd_account_link.into())
			.await
			.map_err(|create_gd_account_link_error| {
				error!(
					"Error creating GD account link record: {}",
					create_gd_account_link_error
				);
				DiscordError::DatabaseError(create_gd_account_link_error)
			})?;

		Ok(DiscordGDAccountLink::new(
			discord_user.discord_user_id,
			gd_player_id,
			gd_username,
			gd_account_challenge
		))
	}

	async fn verify_gd_account_link(&self, discord_user_id: u64) -> Result<(), DiscordError> {
		let mut discord_user = self
			.user_repository
			.get_record(discord_user_id)
			.await
			.map_err(|query_discord_user_error| {
				error!(
					"Error getting user record from database: {}",
					query_discord_user_error
				);
				DiscordError::DatabaseError(query_discord_user_error)
			})?
			.map(DiscordUser::from)
			.map(Ok)
			.unwrap_or_else(|| {
				error!("Discord user with ID {} does not exist", discord_user_id);
				Err(DiscordError::UserDoesNotExist)
			})?;

		if discord_user.gd_player_id.is_some() {
			warn!("Discord account already linked");
			return Err(DiscordError::DiscordAccountAlreadyLinked);
		};

		let fetched_gd_account_link = self
			.gd_account_link_repository
			.get_record(discord_user.discord_user_id)
			.await
			.map_err(|query_gd_account_link_error| {
				error!(
					"Error getting gd account link from database: {}",
					query_gd_account_link_error
				);
				DiscordError::DatabaseError(query_gd_account_link_error)
			})?
			.ok_or_else(|| {
				error!(
					"Discord user with ID {} has not initiated a link",
					discord_user_id
				);
				DiscordError::UserDoesNotExist
			})?;

		if let Some(_) = self
			.gd_account_link_repository
			.get_verified_record_with_gd_player_id(fetched_gd_account_link.gd_player_id)
			.await
			.map_err(|query_gd_account_link_error| {
				error!(
					"Error getting gd account link from database: {}",
					query_gd_account_link_error
				);
				DiscordError::DatabaseError(query_gd_account_link_error)
			})? {
			warn!("GD account is already linked to another user");
			return Err(DiscordError::DiscordAccountAlreadyLinked);
		};

		let gd_account_link = GDAccountLink::from(fetched_gd_account_link);
		let gd_account_id = self
			.geometry_dash_client
			.query_gd_player_and_account_id(&gd_account_link.gd_player_id.to_string())
			.await
			.map_err(|query_gd_account_id_error| {
				error!(
					"Failed to query GD account ID: {}",
					query_gd_account_id_error
				);
				DiscordError::DiscordError
			})?
			.1;
		let gd_public_account_token = self
			.geometry_dash_client
			.get_gd_public_account_token(gd_account_id)
			.await
			.map_err(|get_gd_public_account_token_error| {
				error!(
					"Failed fetching GD public account token: {}",
					get_gd_public_account_token_error
				);
				DiscordError::DiscordError
			})?;

		let is_valid = gd_account_link
			.verify_account_link(&gd_public_account_token)
			.map_err(|verify_account_link_error| {
				if let Some(DiscordError::GDAccountLinkExpired) =
					verify_account_link_error.downcast_ref::<DiscordError>()
				{
					error!("GD account link has expired");
					DiscordError::GDAccountLinkExpired
				} else {
					error!(
						"Error validating gd account link token: {}",
						verify_account_link_error
					);
					DiscordError::DiscordError
				}
			})?;
		if !is_valid {
			error!("Invalid GD account link token");
			return Err(DiscordError::InvalidGDAccountLinkToken);
		}

		let mut updated_link = gd_account_link;
		updated_link.is_gd_account_linked = true;
		discord_user.gd_player_id = Some(updated_link.gd_player_id);

		if let Err(update_gd_account_link_record_error) = self
			.gd_account_link_repository
			.create_or_update_record(updated_link.into())
			.await
		{
			error!(
				"Failed to update GD account link record: {}",
				update_gd_account_link_record_error
			);
			return Err(DiscordError::DatabaseError(
				update_gd_account_link_record_error
			));
		};

		println!("{:?}", discord_user);
		if let Err(update_user_record_error) = self
			.user_repository
			.update_record(discord_user.into())
			.await
		{
			error!("Failed to update user record: {}", update_user_record_error);
			return Err(DiscordError::DatabaseError(update_user_record_error));
		};

		Ok(())
	}
}

impl<'a, U: UserRepository, G: GeometryDashClient> DiscordUserService<'a, U, G> {
	pub fn new(
		user_repository: &'a U,
		gd_account_link_repository: &'a GDAccountLinkRepository,
		geometry_dash_client: &'a G
	) -> Self {
		DiscordUserService {
			user_repository,
			gd_account_link_repository,
			geometry_dash_client
		}
	}
}
