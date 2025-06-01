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
		// Get or create Discord User
		let mut discord_user: DiscordUser;
		match self.user_repository.get_record(discord_user_id).await {
			Ok(Some(fetched_discord_user)) => {
				discord_user = DiscordUser::from(fetched_discord_user);

				if discord_user.is_gd_account_linked {
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
			.query_gd_player_id(&gd_username)
			.await
		{
			Ok(gd_player_id) => {
				discord_user.gd_player_id = Some(gd_player_id);
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

		let mut gd_account_link = GDAccountLink::new(
			discord_user.discord_user_id,
			discord_user.gd_player_id.unwrap()
		);
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
					updated_user.gd_player_id.unwrap(),
					gd_username,
					gd_account_challenge
				))
			}
			Err(update_record_error) => {
				error!("Error updating user record: {}", update_record_error);
				Err(DiscordError::DatabaseError(update_record_error))
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
