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
				let create_table_result = db_conn
					.execute(Statement::from_string(
						db_conn.get_database_backend(),
						format!("CREATE DATABASE IF NOT EXISTS `{}`;", &self.name)
					))
					.await;
				if let Err(err) = create_table_result {
					println!("Couldn't create table {}", err);
					Err(err)
				} else {
					Database::connect(format!("{}/{}", &url, &self.name)).await
				}
			}
			Err(err) => {
				println!("Don't start app {}", err);
				Err(err)
			}
		}
	}
}
