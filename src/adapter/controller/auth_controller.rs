use crate::domain::model::api::auth_api::{AuthApiError, AuthApiRequest, AuthApiResponse};
use crate::domain::model::auth::claims::Claims;
use crate::domain::service::auth_service::AuthService;

#[post("/auth")]
pub fn generate_jwt(authenticating_user: AuthApiRequest) -> Result<AuthApiResponse, AuthApiError>{
    let claims = Claims::new(authenticating_user.discord_app_id);
    let auth_service = AuthService::new(claims);

    match auth_service.generate_jwt() {
        Ok(jwt) => {Ok(AuthApiResponse::new(jwt))}
        Err(err) => {Err(AuthApiError::from(err))}
    }
}