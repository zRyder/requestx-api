use crate::domain::model::{
	error::level_request_error::LevelRequestError, request_rating::RequestRating
};

pub trait RequestService {
	async fn request(
		self,
		level_id: u64,
		youtube_video_link: Option<String>,
		discord_id: u64,
		request_rating: RequestRating
	) -> Result<(), LevelRequestError>;
}
