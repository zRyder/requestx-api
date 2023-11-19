use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
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
		println!("{}", &url);

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
