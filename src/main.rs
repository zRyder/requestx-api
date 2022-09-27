#[macro_use] extern crate rocket;

pub mod model;
pub mod routes;
pub mod utilities;
pub mod tests;

use sqlx::mysql::MySqlPoolOptions;

#[launch]
async fn launch() -> _ {
    dotenv::dotenv().ok();

    let pool = MySqlPoolOptions::new()
        .max_connections(125)
        .connect("mysql://root:Rayuwwe6@localhost:3306/requestxtest?useSSL=false&serverTimezone=UTC")
        .await
        .ok()
        .unwrap();

    rocket::build()
        // .attach(DBConnection::init())
        .manage(pool)
        .mount("/api/v1/users", routes![routes::users::create_user, routes::users::login])
}
