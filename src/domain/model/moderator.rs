use crate::adapter::mysql::model::sea_orm_active_enums;
use crate::adapter::mysql::model::sea_orm_active_enums::Score;
use crate::domain::model::gd_level::RequestRating;

#[derive(Clone, Copy, Debug)]
pub struct Moderator {
    pub level_id: u64,
    pub score: RequestRating,
    pub rating: Rating
}

#[derive(Clone, Copy, Debug)]
pub enum Rating {
    Rate,
    Feature,
    Epic,
    Legendary,
    Mythic
}

impl From<Score> for RequestRating {
    fn from(value: Score) -> Self {
        match value {
            Score::One => {Self::One}
            Score::Two => {Self::Two}
            Score::Three => {Self::Three}
            Score::Four => {Self::Four}
            Score::Five => {Self::Five}
            Score::Six => {Self::Six}
            Score::Seven => {Self::Seven}
            Score::Eight => {Self::Eight}
            Score::Nine => {Self::Nine}
            Score::Ten => {Self::Ten}
        }
    }
}

impl Into<Score> for RequestRating {
    fn into(self) -> Score {
        match self {
            RequestRating::One => {Score::One}
            RequestRating::Two => {Score::Two}
            RequestRating::Three => {Score::Three}
            RequestRating::Four => {Score::Four}
            RequestRating::Five => {Score::Five}
            RequestRating::Six => {Score::Six}
            RequestRating::Seven => {Score::Seven}
            RequestRating::Eight => {Score::Eight}
            RequestRating::Nine => {Score::Nine}
            RequestRating::Ten => {Score::Ten}
        }
    }
}

impl From<sea_orm_active_enums::Rating> for Rating {
    fn from(value: sea_orm_active_enums::Rating) -> Self {
        match value {
            sea_orm_active_enums::Rating::Rate => {Self::Rate}
            sea_orm_active_enums::Rating::Feature => {Self::Feature}
            sea_orm_active_enums::Rating::Epic => {Self::Epic}
            sea_orm_active_enums::Rating::Legendary => {Self::Legendary}
            sea_orm_active_enums::Rating::Mythic => {Self::Mythic}
        }
    }
}

impl Into<sea_orm_active_enums::Rating> for Rating {
    fn into(self) -> sea_orm_active_enums::Rating {
        match self {
            Rating::Rate => {sea_orm_active_enums::Rating::Rate}
            Rating::Feature => {sea_orm_active_enums::Rating::Feature}
            Rating::Epic => {sea_orm_active_enums::Rating::Epic}
            Rating::Legendary => {sea_orm_active_enums::Rating::Legendary}
            Rating::Mythic => {sea_orm_active_enums::Rating::Mythic}
        }
    }
}