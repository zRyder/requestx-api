use crate::domain::model::{
	discord::user::DiscordUser, error::discord::discord_error::DiscordError
};

pub trait UserService {
	async fn get_user(&self, discord_user_id: u64) -> Result<DiscordUser, DiscordError>;
}
