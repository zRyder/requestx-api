use crate::adapter::geometry_dash::geometry_dash_client::GeometryDashClient;
use crate::domain::model::api::auth_api::Auth;
use crate::domain::model::api::gd_api::{GDLevelInfoApiResponseError, GetGDLevelInfoApiResponse};

#[get("/gd/<level_id>")]
pub async fn get_gd_level_info(
    level_id: u64,
    _auth: Auth
) -> Result<GetGDLevelInfoApiResponse, GDLevelInfoApiResponseError> {
    let gd_client = GeometryDashClient::new();

    match gd_client.get_gd_level_info(level_id).await {
        Ok(gd_level_info) => Ok(GetGDLevelInfoApiResponse::from(gd_level_info)),
        Err(get_gd_level_error) => {
            Err(GDLevelInfoApiResponseError::from(get_gd_level_error))
        }
    }
}