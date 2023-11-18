use rocket::serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum RequestRating {
	Easy,
	Normal,
	Hard,
	Harder,
	Insane,
	Demon
}
