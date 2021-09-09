mod hotel;
mod input;
mod output;

use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::error::Error;
use std::path::PathBuf;

use hotel::HotelMap;
use input::InputRow;
use output::OutputRow;

/// Run the conversion asynchronously.
pub async fn run(input: PathBuf, output: PathBuf, hotels: PathBuf) -> Result<(), Box<dyn Error>> {
    // create a hotel map
    let hotels = HotelMap::new(hotels).await?;

    // create a reader
    let mut reader = ReaderBuilder::new().delimiter(b'|').from_path(input)?;

    // create a writer
    let mut writer = WriterBuilder::new().delimiter(b';').from_path(output)?;

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
