use crate::adapter::mysql::model::sea_orm_active_enums;

#[derive(Clone, Copy)]
pub enum RequestRating {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten
}

impl From<sea_orm_active_enums::RequestRating> for RequestRating {
	fn from(value: sea_orm_active_enums::RequestRating) -> Self {
		match value {
			sea_orm_active_enums::RequestRating::One => Self::One,
			sea_orm_active_enums::RequestRating::Two => Self::Two,
			sea_orm_active_enums::RequestRating::Three => Self::Three,
			sea_orm_active_enums::RequestRating::Four => Self::Four,
			sea_orm_active_enums::RequestRating::Five => Self::Five,
			sea_orm_active_enums::RequestRating::Six => Self::Six,
			sea_orm_active_enums::RequestRating::Seven => Self::Seven,
			sea_orm_active_enums::RequestRating::Eight => Self::Eight,
			sea_orm_active_enums::RequestRating::Nine => Self::Nine,
			sea_orm_active_enums::RequestRating::Ten => Self::Ten
		}
	}
}
