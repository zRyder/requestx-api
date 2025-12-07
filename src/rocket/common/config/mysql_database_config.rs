use sea_orm::{Database, DatabaseConnection, DbErr};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MySqlDatabaseConfig {
	user: String,
	password: String,
	host: String,
	port: u16,
	name: String,
}

impl MySqlDatabaseConfig {
	pub async fn configure_mysql_database(&self) -> Result<DatabaseConnection, DbErr> {
		let url = format!(
			"mysql://{}:{}@{}:{}/{}",
			&self.user, &self.password, &self.host, &self.port, &self.name
		);
		Database::connect(&url).await
	}
}
