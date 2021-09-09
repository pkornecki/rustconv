mod hotel;
mod input;
mod output;

use std::error::Error;
use std::path::PathBuf;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};

use hotel::HotelMap;
use input::InputRow;
use output::OutputRow;

/// Run the conversion asynchronously.
pub async fn run() -> Result<(), Box<dyn Error>> {
    // create a hotel map
    let hotels = HotelMap::new(PathBuf::from("hotels.json")).await?;

    // create a reader
    let mut reader = ReaderBuilder::new()
        .delimiter(b'|')
        .from_path(PathBuf::from("input.csv"))?;

    // create a writer
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .from_path(PathBuf::from("output.csv"))?;

    // process each input record
    for record in reader.records() {
        let record = record?;
        // convert to the output
        let output = convert(record, &hotels)?;
        // store the output
        writer.serialize(output)?;
        writer.flush()?;
    }

    Ok(())
}

/// A helper function to convert StringRecord to the OutputRow.
fn convert<'a>(record: StringRecord, hotels: &'a HotelMap) -> Result<OutputRow<'a>, csv::Error> {
    // deserialize the record into InputRow
    let input_row: InputRow = record.deserialize(None)?;
    // create OutputRow based on the InputRow and a HotelMap
    Ok(OutputRow::new(input_row, hotels))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_works() {
        let hotels = create_hotel_map();

        let input = StringRecord::from(vec![
            "BER", "BER00002", "EZ", "BER898", "F", "20180721", "1", "0", "85.50", "IHG",
        ]);
        let expected = OutputRow {
            room_type_and_meal: String::from("EZ F"),
            room_code: String::from("BER898"),
            source: String::from("IHG"),
            hotel_name: "Crowne Plaza Berlin City Centre",
            city_name: "Berlin",
            city_code: String::from("BER"),
            hotel_category: 4.0,
            pax: 1,
            adults: 1,
            children: 0,
            checkin: String::from("2018-07-21"),
            checkout: String::from("2018-07-22"),
            price: String::from("85.50"),
        };

        let actual = convert(input, &hotels).unwrap();
        assert_eq!(expected, actual);
    }

    /// An auxiliary function to create a HotelMap.
    fn create_hotel_map() -> HotelMap {
        let mut result = HotelMap {
            map: HashMap::new(),
        };
        result.add(
            String::from("BER00002"),
            Hotel {
                name: String::from("Crowne Plaza Berlin City Centre"),
                category: 4.0,
                city: String::from("Berlin"),
            },
        );
        result.add(
            String::from("BER00003"),
            Hotel {
                name: String::from("Berlin Marriott Hotel"),
                category: 5.0,
                city: String::from("Berlin"),
            },
        );
        result
    }
}
