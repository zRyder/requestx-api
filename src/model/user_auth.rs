use chrono::{Utc, Duration, NaiveDateTime};
use sqlx::{
    Error, MySql, Pool,
    mysql::MySqlQueryResult,
};

use hmac::{
    Hmac, Mac
};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use argonautica::input::Password;
use sqlx_core::row::Row;
use crate::utilities;

///Struct representing a representing user logging in. This will be used as user provided data to authenticate and preform authorized actions
#[derive(FromForm)]
pub struct LoginUser {
    ///The username of the user who is attempting to login
    pub user_name: String,

    ///The non-encrypted password of the user who is trying to login in
    pub password: String
}

pub struct UserAuth {
    pub user_id: u32,
    pub user_name: String,
    pub is_verified: bool,
    pub issued: NaiveDateTime,
}

impl UserAuth {

    pub fn new(user_id: u32, user_name: String, is_verified: bool, issued: NaiveDateTime) -> Self{
        UserAuth {
            user_id,
            user_name,
            is_verified,
            issued,
        }
    }

    pub async fn insert_user_session(&self, db_conn: &Pool<MySql>, session_id: &String) -> Result<MySqlQueryResult, Error> {
        sqlx::query("INSERT INTO user_sessions VALUES(?, ?, ?)")
            .bind(self.user_id)
            .bind(session_id)
            .bind((self.issued + Duration::hours(4)).to_string())
            .execute(db_conn)
            .await
    }

    pub fn is_valid_session(session_id: &str) {
        print!("{:?}", Some(session_id))
    }

    pub async fn verify_password_hash(&self, db_conn: &Pool<MySql>, password: &String) -> bool {
        match sqlx::query("SELECT PasswordHash FROM user_hash WHERE UserID = ?")
            .bind(self.user_id)
            .fetch_one(db_conn)
            .await {
            Ok(query_result) => {
                match utilities::users::verify_password_hash(query_result.get::<String, usize>(0), &password) {
                    Ok(verified_auth) => {
                        verified_auth
                    }
                    Err(_verify_error) => {
                        false
                    }
                }
            }
            Err(_query_error) => {
                false
            }
        }
    }

    pub fn generate_jwt(&self) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(std::env::var("SECRET_TOKEN").unwrap().as_bytes()).unwrap();
        let mut claims = BTreeMap::new();

        claims.insert("sub", self.user_id.to_string());
        claims.insert("iat", self.issued.to_string());
        claims.insert("exp", (self.issued + Duration::hours(4)).to_string());
        claims.insert("usr",  self.user_name.to_string());
        claims.insert("vrf",  self.is_verified.to_string());

        claims.sign_with_key(&key).unwrap()
    }
}