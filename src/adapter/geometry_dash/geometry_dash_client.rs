use crate::domain::model::{
	error::geometry_dash::geometry_dash_dashrs_error::GeometryDashDashrsError, gd_level::GDLevel,
	moderator::Moderator
};

#[cfg_attr(test, mockall::automock)]
pub trait GeometryDashClient {
	async fn get_gd_level_info(&self, level_id: u64) -> Result<GDLevel, GeometryDashDashrsError>;

	async fn send_gd_level(
		&self,
		moderator_request: Moderator
	) -> Result<(), GeometryDashDashrsError>;
}
