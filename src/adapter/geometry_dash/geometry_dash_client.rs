use crate::domain::model::{
	error::geometry_dash_dashrs_error::GeometryDashDashrsError, gd_level::GDLevel
};

#[cfg_attr(test, mockall::automock)]
pub trait GeometryDashClient {
	async fn get_gd_level_info(self, level_id: u64) -> Result<GDLevel, GeometryDashDashrsError>;
}
