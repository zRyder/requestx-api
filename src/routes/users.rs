use chrono::{Duration, Local, Utc};
use rocket::{
    State,
    time::{
      Duration as RocketDuration, OffsetDateTime
    },
    http::{
        Status, Cookie, CookieJar,
    },
    form::Form,
    serde::json::Json,

};
use crate::{
    model, utilities,
    model::{
        ApiResponse,
    },
};
use sqlx::{
    MySql, Pool,
};

#[post("/create", data="<create_user_form>")]
pub async fn create_user(db_conn: &State<Pool<MySql>>, create_user_form: Form<model::create_user::CreateUser>) -> ApiResponse {

    let json_string;
    let status;

    let mut new_user = model::user::User{
        user_id: 0,
        user_name: create_user_form.user_name.clone(),
        email: create_user_form.email.clone(),
        verified: false,
        created: Utc::now().naive_utc(),
    };

    if !(utilities::users::is_valid_username(&new_user.user_name)) {
        json_string = String::from("{\"message\": \"invalid username\"}");
        status = Status::BadRequest;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }

    if !(utilities::users::is_valid_email(&new_user.email)) {
        json_string = String::from("{\"message\": \"invalid email address\"}");
        status = Status::BadRequest;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }

    if !(utilities::users::is_valid_password(&create_user_form.password)) {
        json_string = String::from("{\"message\": \"invalid password\"}");
        status = Status::BadRequest;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }

    if let Some(_user) = utilities::users::get_user_object_by_name(db_conn, &new_user.user_name).await {
        json_string = String::from("{\"message\": \"username is already in use\"}");
        status = Status::Conflict;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }

    if let Some(_user) = utilities::users::get_user_object_by_email(db_conn, &new_user.email).await {
        json_string = String::from("{\"message\": \"email is already in use\"}");
        status = Status::Conflict;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }

    let password_hash = utilities::users::hash_password(&create_user_form.password);
    loop {
        new_user.user_id = utilities::generate_numeric_id(9);
        return match new_user.create_new_user(db_conn).await {
            Ok(_insert_result) => {
                match new_user.insert_password_hash(db_conn, password_hash).await {
                    Ok(_insert_hash_result) => {
                        json_string = String::from("{\"message\": \"user added successfully\"}");
                        status = Status::Created;
                    }
                    Err(insert_hash_error) => {
                        json_string = String::from(format!("{{\"message\": \"{}\"}}", insert_hash_error.into_database_error().unwrap().to_string()));
                        status = Status::InternalServerError;

                        match new_user.remove_password_hash(db_conn).await {
                            Ok(_remove_result) => {

                            }
                            Err(_remove_error) => {

                            }
                        }
                    }
                }

                ApiResponse {
                    json: Json(json_string),
                    status
                }
            }
            Err(insert_error) => {
                let error_message = insert_error.as_database_error().unwrap().message();
                if !error_message.contains("Duplicate entry") {
                    json_string = String::from(format!("{{\"message\": \"{}\"}}", &insert_error.as_database_error().unwrap().message()));
                    status = Status::InternalServerError;

                    ApiResponse {
                        json: Json(json_string),
                        status
                    }
                }
                else {
                    continue;
                }
            }
        }
    }
}

#[post("/login", data="<login_user_form>")]
pub async fn login(db_conn: &State<Pool<MySql>>, login_user_form: Form<model::user_auth::LoginUser>, mut cookies: &CookieJar<'_>) -> ApiResponse {
    let json_string;
    let status;

    if login_user_form.user_name.is_empty() || login_user_form.password.clone().is_empty() {
        json_string = String::from("{\"message\": \"username or password not provided\"");
        status = Status::BadRequest;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }

    if let Some(user) = utilities::users::get_user_object_by_name(db_conn, &login_user_form.user_name).await {
        let current_time = Utc::now().naive_utc();
        let user_auth = model::user_auth::UserAuth::new(user.user_id, user.user_name, user.verified, current_time);

        if !user_auth.verify_password_hash(db_conn, &login_user_form.password).await {
            json_string = String::from("{\"message\": \"incorrect password\"");
            status = Status::Unauthorized;

            return ApiResponse {
                json: Json(json_string),
                status
            }
        }

        let session_id = utilities::generate_id(256);
        return match user_auth.insert_user_session(db_conn, &session_id).await {
            Ok(_insert_session_result) => {
                cookies.add_private(Cookie::build("sid", session_id)
                    .http_only(true)
                    .secure(true)
                    .expires(OffsetDateTime::now_utc() + RocketDuration::hours(4))
                    .max_age(RocketDuration::hours(4))
                    .finish());

                json_string = String::from(format!("{{message\": \"login successful\", \"token\": {}}}", user_auth.generate_jwt()));
                status = Status::Ok;

                ApiResponse {
                    json: Json(json_string),
                    status
                }
            }
            Err(insert_session_error) => {
                json_string = String::from(format!("{{\"message\": \"{}\"}}", insert_session_error.into_database_error().unwrap().to_string()));
                status = Status::InternalServerError;

                ApiResponse {
                    json: Json(json_string),
                    status
                }
            }
        }
    }
    else {
        json_string = String::from("{\"message\": \"user does not exist\"}");
        status = Status::NotFound;

        return ApiResponse {
            json: Json(json_string),
            status
        }
    }
}