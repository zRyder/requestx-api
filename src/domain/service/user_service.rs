use crate::domain::model::{
	discord::user::{DiscordGDAccountLink, DiscordUser},
	error::discord::discord_error::DiscordError
};

pub trait UserService {
	async fn get_user(&self, discord_user_id: u64) -> Result<DiscordUser, DiscordError>;

	async fn init_gd_account_link(
		&self,
		discord_user_id: u64,
		gd_username: String
	) -> Result<DiscordGDAccountLink, DiscordError>;
}
