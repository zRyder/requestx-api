use crate::domain::model::{
	error::level_request_error::LevelRequestError,
	gd_level::{GDLevelRequest, RequestRating}
};

pub trait RequestService {
	async fn get_level_request(
		&self,
		level_id: u64,
		has_requested_feedback: Option<bool>
	) -> Result<GDLevelRequest, LevelRequestError>;

	async fn make_level_request(
		&self,
		level_id: u64,
		youtube_video_link: String,
		discord_id: u64,
		request_rating: RequestRating,
		has_requested_feedback: bool,
		notify: bool
	) -> Result<GDLevelRequest, LevelRequestError>;

	async fn update_level_request(
		&self,
		level_id: u64,
		discord_user_id: u64,
		youtube_video_link: Option<String>,
		request_rating: Option<RequestRating>,
		has_requested_feedback: Option<bool>,
		notify: Option<bool>
	) -> Result<GDLevelRequest, LevelRequestError>;

	async fn delete_level_request(
		&self,
		level_id: u64
	) -> Result<GDLevelRequest, LevelRequestError>;

	async fn update_level_request_message_id(
		&self,
		level_id: u64,
		discord_message_id: u64
	) -> Result<(), LevelRequestError>;
}
