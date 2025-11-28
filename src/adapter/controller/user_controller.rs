use rocket_framework::State;
use sea_orm::DatabaseConnection;

use crate::{
	adapter::{
		geometry_dash::geometry_dash_client::GeometryDashClient,
		mysql::{
			gd_account_link_repository::GDAccountLinkRepository, user_repository::UserRepository
		}
	},
	domain::{
		model::api::{
			auth_api::Auth,
			user_api::{DiscordUserApiResponseError, GetDiscordUserApiResponse}
		},
		service::{
			discord_user_service::DiscordUserService,
			internal::request_manager_service::RequestManagerService
		}
	}
};

#[get("/user/<discord_user_id>")]
pub async fn get_user(
	db_conn: &State<DatabaseConnection>,
	discord_user_id: u64,
	_auth: Auth
) -> Result<GetDiscordUserApiResponse, DiscordUserApiResponseError> {
	let user_repository = UserRepository::new(db_conn);

	let user_service = DiscordUserService::new(&user_repository);

	match user_service.get_user(discord_user_id).await {
		Ok(discord_user) => {
			let request_cooldown = RequestManagerService {}.get_request_cooldown().await;
			let mut discord_user_response = GetDiscordUserApiResponse::from(discord_user);
			discord_user_response.request_cooldown = request_cooldown;

			Ok(discord_user_response)
		}
		Err(get_discord_user_error) => Err(get_discord_user_error.into())
	}
}

#[post("/user/link", format = "json", data = "<link_gd_account_request_body>")]
pub async fn link_gd_account<'a>(
	db_conn: &State<DatabaseConnection>,
	link_gd_account_request_body: Json<PostLinkGDAccountRequest<'a>>,
	_auth: Auth
) -> Result<PostLinkGDAccountResponse, DiscordUserApiResponseError> {
	let user_repository = UserRepository::new(db_conn);
	let gd_account_link_repository = GDAccountLinkRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();

	let user_service =
		DiscordUserService::new(&user_repository, &gd_account_link_repository, &gd_client);

	match user_service
		.init_gd_account_link(
			link_gd_account_request_body.discord_id,
			link_gd_account_request_body.gd_username.to_string()
		)
		.await
	{
		Ok(response) => Ok(PostLinkGDAccountResponse::from(response)),
		Err(init_gd_account_error) => Err(init_gd_account_error.into())
	}
}

#[get("/user/link/<discord_user_id>")]
pub async fn verify_gd_account_link(
	db_conn: &State<DatabaseConnection>,
	discord_user_id: u64,
	_auth: Auth
) -> Result<(), DiscordUserApiResponseError> {
	let user_repository = UserRepository::new(db_conn);
	let gd_account_link_repository = GDAccountLinkRepository::new(db_conn);
	let gd_client = GeometryDashClient::new();

	let user_service =
		DiscordUserService::new(&user_repository, &gd_account_link_repository, &gd_client);

	match user_service.verify_gd_account_link(discord_user_id).await {
		Ok(_) => Ok(()),
		Err(verify_gd_account_error) => Err(verify_gd_account_error.into())
	}
}
