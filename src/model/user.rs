use chrono::Utc;
use sqlx::{
    Error, MySql, Pool,
    mysql::MySqlQueryResult,
};

#[derive(Default, Debug)]
pub struct User {
    pub user_id: u32,
    pub user_name: String,
    pub email: String,
    pub verified: bool,
    pub created: chrono::NaiveDateTime,
}

impl User {
    pub fn new(user_id: u32, user_name: String, email: String, verified: bool, created: chrono::NaiveDateTime) -> Self {
        User{
            user_id,
            user_name,
            email,
            verified,
            created,
        }
    }

    pub async fn create_new_user(&self, db_conn: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
        sqlx::query("INSERT INTO users VALUES(?, ?, ?, FALSE, ?)")
            .bind(&self.user_id)
            .bind(&self.user_name)
            .bind(&self.email)
            .bind(Utc::now().naive_utc().to_string())
            .execute(db_conn)
            .await
    }

    pub async fn insert_password_hash(&self, db_conn: &Pool<MySql>, password_hash: String) -> Result<MySqlQueryResult, Error> {
        sqlx::query("INSERT INTO user_hash VALUES(?, ?)")
            .bind(self.user_id)
            .bind(password_hash)
            .execute(db_conn)
            .await
    }

    pub async fn remove_password_hash(&self, db_conn: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
        sqlx::query("DELETE FROM user_hash WHERE UserID = ?")
            .bind(self.user_id)
            .execute(db_conn)
            .await
    }
}
