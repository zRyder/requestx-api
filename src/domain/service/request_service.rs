use crate::domain::model::{
	error::level_request_error::LevelRequestError, request_rating::RequestRating
};

pub trait RequestService {
	async fn request<'a>(
		self,
		level_id: u64,
		youtube_video_link: &'a str,
		discord_id: &'a str,
		request_rating: RequestRating
	) -> Result<(), LevelRequestError>;
}
