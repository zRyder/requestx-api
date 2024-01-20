use crate::domain::model::api::level_request_api::LevelRequestApiResponseError;

#[get("/health")]
pub fn get_health() -> Result<(), LevelRequestApiResponseError>{
    Ok(())
}
