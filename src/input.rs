use serde::Deserialize;

/// A struct defining a single row from the input file.
#[derive(Deserialize, PartialEq)]
pub struct InputRow {
    pub city_code: String,
    pub hotel_code: String,
    pub room_type: String,
    pub room_code: String,
    pub meal: String,
    pub checkin: String,
    pub adults: u32,
    pub children: u32,
    pub price: f32,
    pub source: String,
}
