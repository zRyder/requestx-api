use std::fmt::{Display, Formatter};
use chrono::Local;
use rocket_framework::{response, Request, Response};
use rocket_framework::http::{ContentType, Status};
use rocket_framework::response::Responder;
use rocket_framework::serde::json::Json;
use serde::Serialize;
use crate::domain::model::api::level_request_api::LevelLength;
use crate::domain::model::error::geometry_dash::geometry_dash_dashrs_error::GeometryDashDashrsError;
use crate::domain::model::level_request::GDLevel;
use crate::rocket::common::constants::TIMESTAMP_HEADER_NAME;

#[derive(Serialize)]
pub struct GetGDLevelInfoApiResponse {
    pub level_name: String,
    pub level_author: String,
    pub level_length: LevelLength,
}

impl<'r> Responder<'r, 'r> for GetGDLevelInfoApiResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
        let json = Json(self);
        Response::build_from(json.respond_to(&request)?)
            .status(Status::Ok)
            .raw_header("X-Timestamp", format!("{}", Local::now()))
            .header(ContentType::JSON)
            .ok()
    }
}

impl From<GDLevel> for GetGDLevelInfoApiResponse {
    fn from(gd_level: GDLevel) -> Self {
        Self {
            level_name: gd_level.name,
            level_author: gd_level.creator.name,
            level_length: gd_level.level_length.into(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum GDLevelInfoApiResponseError {
    LevelDoesNotExist,
    LevelRequestError
}

impl From<GeometryDashDashrsError> for GDLevelInfoApiResponseError {
    fn from(err: GeometryDashDashrsError) -> Self {
        match err {
            GeometryDashDashrsError::LevelNotFoundError(_) => {
                GDLevelInfoApiResponseError::LevelDoesNotExist
            },
            _ => GDLevelInfoApiResponseError::LevelRequestError
        }
    }
}

impl<'r> Responder<'r, 'r> for GDLevelInfoApiResponseError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
        let json = Json(&self);
        let mut response = Response::build_from(json.respond_to(&request).unwrap());
        response
            .raw_header(TIMESTAMP_HEADER_NAME, format!("{}", Local::now()))
            .header(ContentType::JSON);

        match self {
            GDLevelInfoApiResponseError::LevelDoesNotExist => {
                response.status(Status::NotFound);
            }
            GDLevelInfoApiResponseError::LevelRequestError => {
                response.status(Status::InternalServerError);
            }
        }

        response.ok()
    }
}

impl Display for GDLevelInfoApiResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GDLevelInfoApiResponseError::LevelDoesNotExist => {
                write!(f, "Level does not exist")
            }
            GDLevelInfoApiResponseError::LevelRequestError => {
                write!(f, "Internal server error")
            }
        }
    }
}