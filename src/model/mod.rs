extern crate chrono;

pub mod create_user;
pub mod user_auth;
pub mod user;

use chrono::Utc;
use rocket::{
    http::{
        ContentType, Status
    }, Request,
    Response,
    response::{
        Responder, Result
    },
    serde::json::Json,
};

pub struct ApiResponse {
    pub json: Json<String>,
    pub status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        Response::build_from(self.json.respond_to(&request).unwrap())
            .status(self.status)
            .raw_header("date", format!("{}", Utc::now()))
            .header(ContentType::JSON)
            .ok()
    }
}
