use lazy_static::lazy_static;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use serde_derive::Deserialize;

use crate::rocket::common::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize)]
pub struct MySqlDatabaseConfig {
	user: String,
	password: String,
	host: String,
	port: u16,
	name: String
}

impl MySqlDatabaseConfig {
	pub async fn configure_mysql_database(&self) -> Result<DatabaseConnection, DbErr> {
		let url = format!(
			"mysql://{}:{}@{}:{}",
			&self.user, &self.password, &self.host, &self.port
		);
		let db_conn_result = Database::connect(&url).await;
		debug!("{}", &url);

		match db_conn_result {
			Ok(db_conn) => {
				let create_database_result = db_conn
					.execute(Statement::from_string(
						db_conn.get_database_backend(),
						format!("CREATE DATABASE IF NOT EXISTS `{}`;", &self.name)
					))
					.await;
				if let Err(err) = create_database_result {
					error!("Unable to create database {}", err);
					Err(err)
				} else {
					Database::connect(format!("{}/{}", &url, &self.name)).await
				}
			}
			Err(err) => {
				error!("Unable to connect to database during initialization{}", err);
				Err(err)
			}
		}
	}
}

lazy_static! {
	pub static ref MY_SQL_DATABASE_CONFIG: &'static MySqlDatabaseConfig =
		&APP_CONFIG.mysql_database_config;
}
