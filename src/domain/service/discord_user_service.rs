use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			gd_account_link_repository::GDAccountLinkRepository, model::user::ActiveModel,
			user_repository::UserRepository
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
		match self.user_repository.get_record(discord_user_id).await {
			Ok(Some(discord_user)) => Ok(DiscordUser::from(discord_user)),
			Ok(None) => {
				warn!("Discord user with ID {} does not exist", discord_user_id);
				Err(DiscordError::UserDoesNotExist)
			}
			Err(db_err) => {
				error!("Error getting user record from database: {}", db_err);
				Err(DiscordError::DatabaseError(db_err))
			}
		}
	}

	async fn init_gd_account_link(
		&self,
		discord_user_id: u64,
		gd_username: String
	) -> Result<DiscordGDAccountLink, DiscordError> {
		let discord_user: DiscordUser;
		let gd_player_id: u64;

		match self.user_repository.get_record(discord_user_id).await {
			Ok(Some(fetched_discord_user)) => {
				discord_user = DiscordUser::from(fetched_discord_user);

				if discord_user.gd_player_id.is_some() {
					warn!(
						"Discord account with id {} is already linked to a gd account: {}",
						discord_user.discord_user_id,
						discord_user.gd_player_id.unwrap()
					);
					return Err(DiscordError::DiscordAccountAlreadyLinked);
				}
			}
			Ok(None) => {
				warn!("Discord user with ID {} does not exist", discord_user_id);
				discord_user = DiscordUser::new(discord_user_id);

				let create_record_model: ActiveModel = discord_user.clone().into();
				if let Err(create_record_error) = self
					.user_repository
					.create_record(create_record_model)
					.await
				{
					error!("Error creating user record: {}", create_record_error);
					return Err(DiscordError::DatabaseError(create_record_error));
				};
			}
			Err(db_err) => {
				error!("Error getting user record from database: {}", db_err);
				return Err(DiscordError::DatabaseError(db_err));
			}
		}

		match self
			.geometry_dash_client
			.query_gd_player_and_account_id(&gd_username)
			.await
		{
			Ok((fetched_gd_player_id, _)) => {
				gd_player_id = fetched_gd_player_id;
			}
			Err(query_gd_player_id_error) => {
				return if let GeometryDashDashrsError::UserNotFoundError(_) =
					query_gd_player_id_error
				{
					warn!(
						"Geometry Dash user with username {} does not exist",
						gd_username
					);
					Err(DiscordError::GDAccountDoesNotExist(gd_username))
				} else {
					error!(
						"Error querying gd player_id from Geometry Dash servers: {}",
						query_gd_player_id_error
					);
					Err(DiscordError::DiscordError)
				}
			}
		}

		let query_gd_account_link_result = self
			.gd_account_link_repository
			.get_record(discord_user.discord_user_id)
			.await;

		if let Err(db_err) = query_gd_account_link_result {
			error!("Error querying GD account link from database: {}", db_err);
			return Err(DiscordError::DatabaseError(db_err));
		}
		if let Ok(Some(fetched_gd_account_link)) = query_gd_account_link_result {
			if fetched_gd_account_link.is_gd_account_linked == 1 {
				warn!("GD Account has already been linked to another Discord user");
				return Err(DiscordError::DiscordAccountAlreadyLinked);
			}
		};

		let mut gd_account_link = GDAccountLink::new(discord_user.discord_user_id, gd_player_id);
		gd_account_link.generate_new_account_link();
		let gd_account_challenge = gd_account_link.gd_account_challenge.clone();
		match self
			.user_repository
			.update_record(discord_user.into())
			.await
		{
			Ok(updated_user) => {
				if let Err(gd_account_link_db_error) = self
					.gd_account_link_repository
					.create_or_update_record(gd_account_link.into())
					.await
				{
					error!(
						"Error writing gd account link to database: {}",
						gd_account_link_db_error
					);
					return Err(DiscordError::DatabaseError(gd_account_link_db_error));
				};

				Ok(DiscordGDAccountLink::new(
					updated_user.discord_id,
					gd_player_id,
					gd_username,
					gd_account_challenge
				))
			}
			Err(update_record_error) => {
				error!(
					"Error updating Discord user record: {}",
					update_record_error
				);
				Err(DiscordError::DatabaseError(update_record_error))
			}
		}
	}

	// Check that GD account isn't already linked to someone else
	async fn verify_gd_account_link(&self, discord_user_id: u64) -> Result<(), DiscordError> {
		let mut discord_user: DiscordUser;
		let mut gd_account_link: GDAccountLink;
		let fetched_user_challenge: String;
		match self.user_repository.get_record(discord_user_id).await {
			Ok(Some(fetched_discord_user)) => {
				if fetched_discord_user.gd_player_id.is_some() {
					warn!(
						"Discord account has already been linked to another Geometry Dash account"
					);
					return Err(DiscordError::DiscordAccountAlreadyLinked);
				}
				discord_user = DiscordUser::from(fetched_discord_user);
			}
			Ok(None) => {
				error!("Discord user with ID {} does not exist", discord_user_id);
				return Err(DiscordError::UserDoesNotExist);
			}
			Err(query_discord_user_error) => {
				error!(
					"Error getting user record from database: {}",
					query_discord_user_error
				);
				return Err(DiscordError::DatabaseError(query_discord_user_error));
			}
		}

		match self
			.gd_account_link_repository
			.get_record(discord_user.discord_user_id)
			.await
		{
			Ok(Some(fetched_gd_account_link)) => {
				match self
					.gd_account_link_repository
					.get_verified_record_with_gd_player_id(fetched_gd_account_link.gd_player_id)
					.await
				{
					Ok(Some(_)) => {
						warn!(
							"Geometry Dash account has already been linked to another Discord user"
						);
						return Err(DiscordError::DiscordAccountAlreadyLinked);
					}
					Ok(None) => {
						gd_account_link = GDAccountLink::from(fetched_gd_account_link);
					}
					Err(query_gd_account_link_error) => {
						error!(
							"Error querying gd account_id from Geometry Dash servers: {}",
							query_gd_account_link_error
						);
						return Err(DiscordError::DiscordError);
					}
				}
			}
			Ok(None) => {
				return if discord_user.gd_player_id.is_some() {
					error!(
						"Discord user with ID {} has already linked a GD account",
						discord_user_id
					);
					Err(DiscordError::DiscordAccountAlreadyLinked)
				} else {
					error!(
						"Discord user with ID {} has not initiated a link",
						discord_user_id
					);
					Err(DiscordError::UserDoesNotExist)
				}
			}
			Err(query_discord_gd_account_link_error) => {
				error!(
					"Error querying database for GD account link data: {}",
					query_discord_gd_account_link_error
				);
				return Err(DiscordError::DatabaseError(
					query_discord_gd_account_link_error
				));
			}
		}

		let gd_account_id: u64;
		match self
			.geometry_dash_client
			.query_gd_player_and_account_id(&gd_account_link.gd_player_id.to_string())
			.await
		{
			Ok((_, fetched_gd_account_id)) => {
				gd_account_id = fetched_gd_account_id;
			}
			Err(query_gd_account_id_error) => {
				error!(
					"Error querying gd account_id from Geometry Dash servers: {}",
					query_gd_account_id_error
				);
				return Err(DiscordError::DiscordError);
			}
		}
		match self
			.geometry_dash_client
			.get_gd_public_account_token(gd_account_id)
			.await
		{
			Ok(gd_public_account_token) => {
				fetched_user_challenge = gd_public_account_token;
			}
			Err(err) => {
				error!("Error getting gd public account token: {}", err);
				return Err(DiscordError::DiscordError);
			}
		};

		match gd_account_link.verify_account_link(&fetched_user_challenge) {
			Ok(is_valid) => {
				if is_valid {
					gd_account_link.is_gd_account_linked = true;
					discord_user.gd_player_id = Some(gd_account_link.gd_player_id);

					if let Err(update_error) = self
						.gd_account_link_repository
						.create_or_update_record(gd_account_link.into())
						.await
					{
						error!(
							"Error updating gd account link to database: {}",
							update_error
						);
						return Err(DiscordError::DatabaseError(update_error));
					}
					if let Err(update_error) = self
						.user_repository
						.update_record(discord_user.into())
						.await
					{
						error!(
							"Error updating gd account link to database: {}",
							update_error
						);
						return Err(DiscordError::DatabaseError(update_error));
					}
					Ok(())
				} else {
					Err(DiscordError::InvalidGDAccountLinkToken)
				}
			}
			Err(err) => {
				if let Some(DiscordError::GDAccountLinkExpired) = err.downcast_ref::<DiscordError>()
				{
					error!("GD account link token has expired");
					Err(DiscordError::GDAccountLinkExpired)
				} else {
					error!("Error validating gd account link token: {}", err);
					Err(DiscordError::DiscordError)
				}
			}
		}
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
