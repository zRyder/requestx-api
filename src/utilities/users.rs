use argonautica::{
    Hasher, Verifier
};
use regex::Regex;

use sqlx::{
    Error, MySql, Pool,
};
use sqlx_core::mysql::MySqlRow;
use sqlx_core::row::Row;
use crate::model;

pub async fn get_user_object_by_id(db_conn: &Pool<MySql>, user_id: &u32) -> Option<model::user::User> {
    match get_user_by_id(db_conn, user_id).await {
        Ok(query_result) => {
            Some(model::user::User::new(
                query_result.get::<u32, usize>(0),
                query_result.get::<String, usize>(1),
                query_result.get::<String, usize>(2),
                query_result.get::<bool, usize>(3),
                query_result.get::<chrono::NaiveDateTime, usize>(4)
            ))
        }
        Err(_query_error) => {
            //Log error
            None
        }
    }
}

async fn get_user_by_id(db_conn: &Pool<MySql>, user_id: &u32) -> Result<MySqlRow, Error> {
    sqlx::query("SELECT * FROM users WHERE UserID = ?")
        .bind(user_id)
        .fetch_one(db_conn)
        .await
}

pub async fn get_user_object_by_name(db_conn: &Pool<MySql>, user_name: &String) -> Option<model::user::User> {
    match get_user_by_name(db_conn, user_name).await {
        Ok(query_result) => {
            Some(model::user::User::new(
                query_result.get::<u32, usize>(0),
                query_result.get::<String, usize>(1),
                query_result.get::<String, usize>(2),
                query_result.get::<bool, usize>(3),
                query_result.get::<chrono::NaiveDateTime, usize>(4)
            ))
        }
        Err(_query_error) => {
            //Log error
            None
        }
    }
}

async fn get_user_by_name(db_conn: &Pool<MySql>, user_name: &String) -> Result<MySqlRow, Error> {
    sqlx::query("SELECT * FROM users WHERE UserName = ?")
        .bind(user_name)
        .fetch_one(db_conn)
        .await
}

pub async fn get_user_object_by_email(db_conn: &Pool<MySql>, email: &String) -> Option<model::user::User> {
    match get_user_by_email(db_conn, email).await {
        Ok(query_result) => {
            Some(model::user::User::new(
                query_result.get::<u32, usize>(0),
                query_result.get::<String, usize>(1),
                query_result.get::<String, usize>(2),
                query_result.get::<bool, usize>(3),
                query_result.get::<chrono::NaiveDateTime, usize>(4)
            ))
        }
        Err(_query_error) => {
            //Log error
            None
        }
    }
}

async fn get_user_by_email(db_conn: &Pool<MySql>, email: &String) -> Result<MySqlRow, Error> {
    sqlx::query("SELECT * FROM users WHERE Email = ?")
        .bind(email)
        .fetch_one(db_conn)
        .await
}

pub fn is_valid_username(user_name: &String) -> bool {
    if (user_name.chars().all(char::is_alphanumeric)) && (user_name.len() >= 3) {
        //CHECK FOR BANNED USERNAMES HERE

        true
    }
    else {
        false
    }
}

pub fn is_valid_email(email: &String) -> bool {
    //THIS REGEX WILL VALIDATE EMAIL ADDRESSES DO NOT CHANGE
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    if email_regex.is_match(&*email.to_lowercase()) {
        true
    }
    else {
        false
    }
}

pub fn is_valid_password(password: &String) -> bool {
    if password.len() >= 8 && has_symbol(password) && has_number(password) && has_capital_letter(password) {
        true
    }
    else {
        false
    }
}

fn has_number(password: &String) -> bool {
    for character in password.chars() {
        if character.is_numeric() {
            return true
        }
    }
    false
}

fn has_symbol(password: &String)-> bool {
    for character in password.chars() {
        if !(character.is_alphanumeric()) {
            return true
        }
    }
    false
}

fn has_capital_letter(password: &String) -> bool {
    for character in password.chars() {
        if character.is_uppercase() {
            return true
        }
    }
    false
}

pub fn hash_password(password: &String) -> String
{
    dotenv::dotenv().ok();
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .configure_secret_key_clearing(true)
        .with_secret_key(std::env::var("SECRET_HASH").unwrap())
        .hash()
        .unwrap();

    hash
}

pub fn verify_password_hash(password_hash: String, password: &String) -> Result<bool, argonautica::Error> {
    dotenv::dotenv().ok();

    //TO VERIFY PASSWORDS
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(password_hash)
        .with_password(password)
        .configure_secret_key_clearing(true)
        .with_secret_key(std::env::var("SECRET_HASH").unwrap())
        .verify();

    is_valid
}
