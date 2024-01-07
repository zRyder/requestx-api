use sea_orm::ActiveValue;
use crate::adapter::mysql::model::review;

#[derive(Debug, Clone)]
pub struct Review {
    pub reviewer_discord_id: u64,
    pub discord_message_id: u64,
    pub level_id: u64,
    pub review_contents: String,
    pub is_update: bool
}

impl Into<review::ActiveModel> for Review {
    fn into(self) -> review::ActiveModel {
        review::ActiveModel {
            level_id: ActiveValue::Set(self.level_id),
            discord_id: ActiveValue::Set(self.reviewer_discord_id),
            message_id: ActiveValue::Set(self.discord_message_id),
            review_content: ActiveValue::Set(self.review_contents),
        }
    }
}