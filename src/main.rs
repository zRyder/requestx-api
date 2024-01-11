#[macro_use]
extern crate rocket as rocket_framework;

mod adapter;
mod domain;

mod rocket;

use crate::{
	adapter::controller::{level_request_controller, level_review_controller, reviewer_controller},
	rocket::common::{
		config::common_config::AppConfig, internal::internal::mount_internal_controllers
	}
};

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

	let rocket = rocket.manage(db_conn).mount(
		"/api/v1",
		routes![
			level_request_controller::request_level,
			level_request_controller::get_level_request,
			level_review_controller::get_level_review,
			level_review_controller::review_level,
			reviewer_controller::get_reviewer,
			reviewer_controller::create_reviewer,
			reviewer_controller::remove_reviewer,
		]
	);
	mount_internal_controllers(rocket)
}
