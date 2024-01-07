use crate::domain::model::{
	error::level_request_error::LevelRequestError, request_rating::RequestRating
};
use crate::domain::model::gd_level::GDLevelRequest;

pub trait RequestService {
	async fn request(
		self,
		level_id: u64,
		youtube_video_link: String,
		discord_id: u64,
		request_rating: RequestRating
	) -> Result<GDLevelRequest, LevelRequestError>;
}
