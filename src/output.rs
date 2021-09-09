use chrono::{Duration, NaiveDate};
use serde::Serialize;

use crate::hotel::HotelMap;
use crate::input::InputRow;

/// A struct defining a single row from in the output file.
#[derive(Debug, Serialize, PartialEq)]
pub struct OutputRow<'a> {
    #[serde(rename = "room_type meal")]
    room_type_and_meal: String,
    room_code: String,
    source: String,
    hotel_name: &'a str,
    city_name: &'a str,
    city_code: String,
    hotel_category: f32,
    pax: u32,
    adults: u32,
    children: u32,
    checkin: String,
    checkout: String,
    price: String,
}

impl<'a> OutputRow<'a> {
    /// Create a new OutputRow.
    ///
    /// Takes InputRow and HotelMap as parameters.
    pub fn new<'b>(input: InputRow, hotels: &'b HotelMap) -> OutputRow<'b> {
        OutputRow {
            room_type_and_meal: format!("{} {}", input.room_type, input.meal),
            room_code: input.room_code.clone(),
            source: input.source.clone(),
            hotel_name: hotels.get_name(&input.hotel_code),
            city_name: hotels.get_city_name(&input.hotel_code),
            city_code: input.city_code,
            hotel_category: hotels.get_category(&input.hotel_code),
            pax: input.adults + input.children,
            adults: input.adults,
            children: input.children,
            checkin: Self::get_checkin(&input.checkin),
            checkout: Self::get_checkout(&input.checkin),
            price: Self::calculate_price(input.price, input.adults, input.children),
        }
    }

    /// Get the checkin date as a String.
    ///
    /// If the argument is not a valid date, "???" is returned.
    fn get_checkin(checkin: &str) -> String {
        if let Ok(date) = NaiveDate::parse_from_str(checkin, "%Y%m%d") {
            format!("{}", date)
        } else {
            eprintln!("error: invalid date: {}", checkin);
            String::from("???")
        }
    }

    /// Get the checkout data as a String.
    ///
    /// The checkout is one day after the date specified in the argument.
    /// If the argument is not a valid date, "???" is returned.
    fn get_checkout(checkin: &str) -> String {
        if let Ok(date) = NaiveDate::parse_from_str(checkin, "%Y%m%d") {
            format!("{}", date + Duration::days(1))
        } else {
            eprintln!("error: invalid date: {}", checkin);
            String::from("???")
        }
    }

    /// Calculate the price per person.
    fn calculate_price(price: f32, adults: u32, children: u32) -> String {
        let price = price * 100.0 / (adults + children) as f32;
        format!("{:.2}", price.ceil() / 100.0)
    }
}
