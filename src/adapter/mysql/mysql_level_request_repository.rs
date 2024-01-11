use sea_orm::{DatabaseConnection, DbConn, DbErr, DeleteResult, EntityTrait, InsertResult};

use crate::adapter::mysql::{
	level_request_repository::LevelRequestRepository,
	model::{level_request::ActiveModel, prelude::*, *}
};

pub struct MySqlLevelRequestRepository<'a> {
	db_conn: &'a DatabaseConnection
}

impl<'a> LevelRequestRepository for MySqlLevelRequestRepository<'a> {
	async fn create_record(self, record: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
		LevelRequest::insert(record).exec(self.db_conn).await
	}

	async fn get_record(&self, level_id: u64) -> Result<Option<level_request::Model>, DbErr> {
		LevelRequest::find_by_id(level_id).one(self.db_conn).await
	}

	async fn update_record(self, record: ActiveModel) -> Result<level_request::Model, DbErr> {
		LevelRequest::update(record).exec(self.db_conn).await
	}

	async fn delete_record(self, record: ActiveModel) -> Result<DeleteResult, DbErr> {
		LevelRequest::delete(record).exec(self.db_conn).await
	}
}

impl<'a> MySqlLevelRequestRepository<'a> {
	pub fn new(db_conn: &'a DbConn) -> Self { MySqlLevelRequestRepository { db_conn } }
}

// #[cfg(test)]
// mod tests {
// 	use rocket::tokio;
// 	use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseBackend, MockDatabase,
// MockExecResult};
//
// 	use crate::adapter::mysql::{
// 		level_request_repository::LevelRequestRepository,
// 		model::{level_request, sea_orm_active_enums::RequestRating},
// 		mysql_level_request_repository::MySqlLevelRequestRepository
// 	};
//
// 	#[tokio::test]
// 	async fn test_insert_level_record_should_insert_properly() {
// 		let db = MockDatabase::new(DatabaseBackend::MySql)
// 			.append_exec_results([MockExecResult {
// 				last_insert_id: 1,
// 				rows_affected: 1
// 			}])
// 			.into_connection();
//
// 		let level_request_storable = setup_helper_record(
// 			99999999,
// 			99999999,
// 			"Level Name".to_string(),
// 			Some("Level Description".to_string()),
// 			"Level Creator".to_string(),
// 			RequestRating::Two,
// 			None
// 		);
//
// 		let repository = MySqlLevelRequestRepository { db_conn: &db };
//
// 		match repository.create_record(level_request_storable).await {
// 			Ok(result) => {
// 				assert_eq!(result.last_insert_id, 99999999)
// 			}
// 			Err(_err) => {
// 				assert!(false)
// 			}
// 		}
// 	}
//
// 	#[tokio::test]
// 	async fn test_update_level_record_should_update_properly() {
// 		let initial_result = level_request::Model {
// 			level_id: 99999999,
// 			discord_id: 0,
// 			discord_message_id: None,
// 			discord_thread_id: None,
// 			name: "Level Name".to_string(),
// 			description: Some("Level Description".to_string()),
// 			author: "Level Creator".to_string(),
// 			request_rating: RequestRating::Two,
// 			you_tube_video_link: "None".to_string()
// 		};
// 		let updated_result = level_request::Model {
// 			level_id: 99999999,
// 			discord_id: 0,
// 			discord_message_id: None,
// 			discord_thread_id: None,
// 			name: "Level Name".to_string(),
// 			description: Some("New Level Description".to_string()),
// 			author: "New Level Creator".to_string(),
// 			request_rating: RequestRating::Two,
// 			you_tube_video_link: "None".to_string()
// 		};
//
// 		let db = MockDatabase::new(DatabaseBackend::MySql)
// 			.append_query_results([[initial_result], [updated_result.clone()]])
// 			.append_exec_results([
// 				MockExecResult {
// 					last_insert_id: 99999999,
// 					rows_affected: 1
// 				},
// 				MockExecResult {
// 					last_insert_id: 99999999,
// 					rows_affected: 1
// 				}
// 			])
// 			.into_connection();
//
// 		let initial_record = setup_helper_record(
// 			99999999,
// 			99999999,
// 			"Level Name".to_string(),
// 			Some("Level Description".to_string()),
// 			"Level Creator".to_string(),
// 			RequestRating::Two,
// 			None
// 		);
//
// 		initial_record
// 			.insert(&db)
// 			.await
// 			.expect("Failed initial insert");
//
// 		let level_request_storable = setup_helper_record(
// 			99999999,
// 			99999999,
// 			"New Level Name".to_string(),
// 			Some("New Level Description".to_string()),
// 			"New Level Creator".to_string(),
// 			RequestRating::Four,
// 			None
// 		);
//
// 		let repository = MySqlLevelRequestRepository { db_conn: &db };
//
// 		match repository.update_record(level_request_storable).await {
// 			Ok(result) => {
// 				assert_eq!(result, updated_result)
// 			}
// 			Err(err) => {
// 				println!("{:?}", err);
// 				assert!(false)
// 			}
// 		}
// 	}
//
// 	#[tokio::test]
// 	async fn test_delete_level_record_should_delete_properly() {
// 		let initial_result = level_request::Model {
// 			level_id: 99999999,
// 			discord_id: 0,
// 			discord_message_id: None,
// 			discord_thread_id: None,
// 			name: "Level Name".to_string(),
// 			description: Some("Level Description".to_string()),
// 			author: "Level Creator".to_string(),
// 			request_rating: RequestRating::Two,
// 			you_tube_video_link: None
// 		};
//
// 		let db = MockDatabase::new(DatabaseBackend::MySql)
// 			.append_query_results([[initial_result]])
// 			.append_exec_results([
// 				MockExecResult {
// 					last_insert_id: 99999999,
// 					rows_affected: 1
// 				},
// 				MockExecResult {
// 					last_insert_id: 99999999,
// 					rows_affected: 1
// 				}
// 			])
// 			.into_connection();
//
// 		let initial_record = setup_helper_record(
// 			99999999,
// 			99999999,
// 			"Level Name".to_string(),
// 			Some("Level Description".to_string()),
// 			"Level Creator".to_string(),
// 			RequestRating::Two,
// 			None
// 		);
//
// 		initial_record
// 			.insert(&db)
// 			.await
// 			.expect("Failed initial insert");
//
// 		let level_request_storable = setup_helper_record(
// 			99999999,
// 			99999999,
// 			"Level Name".to_string(),
// 			Some("Level Description".to_string()),
// 			"Level Creator".to_string(),
// 			RequestRating::Two,
// 			None
// 		);
//
// 		let repository = MySqlLevelRequestRepository { db_conn: &db };
//
// 		match repository.delete_record(level_request_storable).await {
// 			Ok(result) => {
// 				assert_eq!(result.rows_affected, 1)
// 			}
// 			Err(err) => {
// 				println!("{:?}", err);
// 				assert!(false)
// 			}
// 		}
// 	}
//
// 	fn setup_helper_record(
// 		id: u64,
// 		discord_id: u64,
// 		name: String,
// 		description: Option<String>,
// 		author: String,
// 		request_rating: RequestRating,
// 		you_tube_video_link: String
// 	) -> level_request::ActiveModel {
// 		level_request::ActiveModel {
// 			level_id: ActiveValue::Set(id),
// 			discord_id: ActiveValue::Set(discord_id),
// 			discord_message_id: ActiveValue::Set(None),
// 			discord_thread_id: ActiveValue::Set(None),
// 			name: ActiveValue::Set(name),
// 			description: ActiveValue::Set(description),
// 			author: ActiveValue::Set(author),
// 			request_rating: ActiveValue::Set(request_rating),
// 			you_tube_video_link: ActiveValue::Set(you_tube_video_link)
// 		}
// 	}
// }
