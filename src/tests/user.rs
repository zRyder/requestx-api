#![allow(unused)]

use sqlx::{
    mysql::MySqlPoolOptions
};
use crate::model;

macro_rules! blocking_await {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}

#[test]
pub fn get_user_by_name_test() {
    let pool = blocking_await!(
        MySqlPoolOptions::new()
        .connect("mysql://root:Rayuwwe6@localhost:3306/requestxtest?useSSL=false&serverTimezone=UTC"))
        .ok()
        .unwrap();

    blocking_await!(sqlx::query("INSERT INTO users VALUES(1, ?, ?, TRUE, ?)")
        .bind("testUser")
        .bind("test@someemail.com")
        .bind(chrono::Local::now().naive_utc().to_string())
        .execute(&pool));

    if let Some(test) = blocking_await!(crate::utilities::users::get_user_object_by_name(&pool, &String::from("testUser"))) {
        println!("{:?}", test)
    }
    else {
        assert!(false)
    }

}

#[test]
pub fn create_new_user_test() {
    let pool = blocking_await!(
        MySqlPoolOptions::new()
        .connect("mysql://root:Rayuwwe6@localhost:3306/requestxtest?useSSL=false&serverTimezone=UTC"))
        .ok()
        .unwrap();

    let result = blocking_await!(sqlx::query("INSERT INTO users VALUES(1, ?, ?, TRUE, ?)")
        .bind("testUser")
        .bind("test@someemail.com")
        .bind(chrono::Local::now().naive_utc().to_string())
        .execute(&pool));

    match result {
        Ok(_thing) => {
            assert!(true)
        }
        Err(error) => {
            println!("{:?}", error.into_database_error().unwrap().message())
        }
    }
}

#[test]
pub fn generate_jwt_test() {
    let test_user = model::user_auth::UserAuth::new(1, String::from("test"), true, chrono::Utc::now().naive_utc());
    println!("{}", test_user.generate_jwt())
}