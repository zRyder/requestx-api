#[macro_use]
extern crate rocket as rocket_framework;

mod adapter;
mod domain;

mod rocket;

use crate::{
	adapter::mysql::level_request_repository::LevelRequestRepository,
	domain::service::request_service::RequestService,
	rocket::common::config::common_config::AppConfig
};

#[launch]
async fn launch() -> _ {
	dotenv::dotenv().ok();
	let rocket = rocket_framework::build();
	let figment = rocket.figment();

	let app_config: AppConfig = figment.extract().expect("Some");
	println!("{:?}", app_config);
	let db_conn = match app_config
		.mysql_database_config
		.configure_mysql_database()
		.await
	{
		Ok(conn) => conn,
		Err(err) => {
			panic!("{}", err)
		}
	};

	rocket.manage(db_conn)
	// .mount("/api/v1",
	// routes![adapter::controller::level_request_controller::request_level])
}
