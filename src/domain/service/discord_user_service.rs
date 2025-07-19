use crate::{
	adapter::mysql::user_repository::UserRepository,
	domain::{
		model::{discord::user::DiscordUser, error::discord::discord_error::DiscordError},
		service::user_service::UserService
	}
};

pub struct DiscordUserService<'a, U: UserRepository> {
	user_repository: &'a U
}

impl<'a, U: UserRepository> UserService for DiscordUserService<'a, U> {
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
}

impl<'a, U: UserRepository> DiscordUserService<'a, U> {
	pub fn new(user_repository: &'a U) -> Self { DiscordUserService { user_repository } }
}
