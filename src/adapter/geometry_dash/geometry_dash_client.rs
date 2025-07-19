use crate::domain::model::{
	error::geometry_dash::geometry_dash_dashrs_error::GeometryDashDashrsError,
	level_request::GDLevel, moderator::Moderator
};

#[cfg_attr(test, mockall::automock)]
pub trait GeometryDashClient {
	async fn get_gd_level_info(&self, level_id: u64) -> Result<GDLevel, GeometryDashDashrsError>;

	async fn query_gd_player_and_account_id(
		&self,
		gd_username: &str
	) -> Result<(u64, u64), GeometryDashDashrsError>;

	async fn send_gd_level(
		&self,
		moderator_request: Moderator
	) -> Result<(), GeometryDashDashrsError>;

	async fn get_gd_public_account_token(
		&self,
		account_id: u64
	) -> Result<String, GeometryDashDashrsError>;
}
