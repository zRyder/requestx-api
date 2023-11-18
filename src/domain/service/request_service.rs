use crate::domain::model::error::level_request_error::LevelRequestError;

pub trait RequestService {
	async fn request(self) -> Result<(), LevelRequestError>;
}
