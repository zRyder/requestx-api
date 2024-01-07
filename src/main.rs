#[macro_use]
extern crate rocket as rocket_framework;

mod adapter;
mod domain;

mod rocket;

use crate::rocket::common::config::common_config::AppConfig;

#[launch]
async fn launch() -> _ {
	log4rs::init_file("log4rs.yml", Default::default()).unwrap();
	info!("Starting requestx-api");
	let rocket = rocket_framework::build();
	let figment = rocket.figment();

	info!("Initializing application configuration");
	let app_config: AppConfig = figment.extract().expect("Some");

	info!("Initializing database");
	let db_conn = match app_config
		.mysql_database_config
		.configure_mysql_database()
		.await
	{
		Ok(conn) => conn,
		Err(err) => {
			error!("Failed to initialize database: {}", err);
			panic!("{}", err)
		}
	};

	rocket.manage(db_conn).mount(
		"/api/v1",
		routes![adapter::controller::level_request_controller::request_level]
	)
}
