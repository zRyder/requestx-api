#[macro_use]
extern crate rocket as rocket_framework;

mod adapter;
mod domain;

mod rocket;

use rocket_framework::Config;
use crate::{
	adapter::controller::{
		auth_controller, level_request_controller, level_review_controller, reviewer_controller, health
	},
	rocket::common::{
		config::{common_config::init_app_config, mysql_database_config::MY_SQL_DATABASE_CONFIG},
		internal::internal::mount_internal_controllers
	}
};
use crate::rocket::common::config::common_config::APP_CONFIG;

#[launch]
async fn launch() -> _ {
	log4rs::init_file("log4rs.yml", Default::default()).unwrap();
	info!("Starting requestx-api");

	info!("Initializing application configuration");
	if let Err(err) = init_app_config() {
		error!("Failed to load app config: {}", err);
		panic!("{}", err)
	}

	info!("Initializing database");
	let db_conn = match MY_SQL_DATABASE_CONFIG.configure_mysql_database().await {
		Ok(conn) => conn,
		Err(err) => {
			error!("Failed to initialize database: {}", err);
			panic!("{}", err)
		}
	};

	let mut rocket = rocket_framework::custom(
		Config::figment()
			.merge(("address", &APP_CONFIG.client_config.host))
			.merge(("port", &APP_CONFIG.client_config.port))
	);

	rocket = rocket.manage(db_conn).mount(
		"/api/v1",
		routes![
			auth_controller::generate_jwt,
			level_request_controller::request_level,
			level_request_controller::get_level_request,
			level_review_controller::get_level_review,
			level_review_controller::review_level,
			reviewer_controller::get_reviewer,
			reviewer_controller::create_reviewer,
			reviewer_controller::remove_reviewer,
			health::get_health
		]
	);
	mount_internal_controllers(rocket)
}
