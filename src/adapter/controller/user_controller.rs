use rocket_framework::State;
use sea_orm::DatabaseConnection;

use crate::{
	adapter::mysql::mysql_user_repository::MySqlUserRepository,
	domain::{
		model::api::{
			auth_api::Auth,
			user_api::{DiscordUserApiResponseError, GetDiscordUserApiResponse}
		},
		service::{discord_user_service::DiscordUserService, user_service::UserService}
	}
};

#[get("/user/<discord_user_id>")]
pub async fn get_user(
	db_conn: &State<DatabaseConnection>,
	discord_user_id: u64,
	_auth: Auth
) -> Result<GetDiscordUserApiResponse, DiscordUserApiResponseError> {
	let user_repository = MySqlUserRepository::new(db_conn);

	let user_service = DiscordUserService::new(&user_repository);

	match user_service.get_user(discord_user_id).await {
		Ok(discord_user) => Ok(GetDiscordUserApiResponse::from(discord_user)),
		Err(get_discord_user_error) => Err(get_discord_user_error.into())
	}
}
