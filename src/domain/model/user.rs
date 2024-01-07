use sea_orm::ActiveValue;
use crate::adapter::mysql::model::user;

#[derive(Clone, Copy)]
pub struct User {
    pub discord_id: u64,
}

impl Into<user::ActiveModel> for User{
    fn into(self) -> user::ActiveModel {
        user::ActiveModel{
            discord_id: ActiveValue::Set(self.discord_id),
        }
    }
}