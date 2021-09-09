use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::path::PathBuf;

use serde::Deserialize;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;

/// A struct that maps the id to the Hotel entry
pub struct HotelMap {
    map: HashMap<String, Hotel>,
}

impl HotelMap {
    /// Create a new HotelMap asynchronously.
    ///
    /// A file_name provides the name of the file, where the information
    /// about the hotels is stored
    pub async fn new(file_name: PathBuf) -> Result<HotelMap, Box<dyn Error>> {
        let file = File::open(file_name).await?;
        let reader = BufReader::new(file);
        let mut result = HotelMap {
            map: HashMap::new(),
        };

        let (tx, mut rx) = mpsc::channel(100);

        let mut lines = reader.lines();
        let handle = tokio::spawn(async move {
            let tx = tx.clone();
            while let Some(row) = lines.next_line().await? {
                let tx = tx.clone();
                tokio::spawn(async move {
                    let hotel = Hotel::parse(&row);
                    tx.send(hotel).await?;
                    Ok::<(), mpsc::error::SendError<(String, Hotel)>>(())
                });
            }
            drop(tx); // close sending channel
            Ok::<(), io::Error>(())
        });

        while let Some(hotel) = rx.recv().await {
            let (id, hotel) = hotel;
            result.add(id, hotel);
        }

        let _ = handle.await?;
        Ok(result)
    }

    /// Get the name of the hotel with a given id
    ///
    /// If the hotel is not found, "???" is returned.
    pub fn get_name(&self, id: &str) -> &str {
        match self.map.get(id) {
            Some(hotel) => &hotel.name,
            None => {
                eprintln!("can't find the name of hotel with the given id: {}", id);
                "???"
            }
        }
    }

    /// Get the category of the hotel with a given id
    ///
    /// If the hotel is not found, 0.0 is returned
    pub fn get_category(&self, id: &str) -> f32 {
        match self.map.get(id) {
            Some(hotel) => hotel.category,
            None => {
                eprintln!("can't find the category of hotel with the given id: {}", id);
                0.0
            }
        }
    }

    /// Get the city of the hotel with a given id
    ///
    /// If the hotel is not found, "???" is returned
    pub fn get_city_name(&self, id: &str) -> &str {
        match self.map.get(id) {
            Some(hotel) => &hotel.city,
            None => {
                eprintln!("can't find the city of hotel with the given id: {}", id);
                "???"
            }
        }
    }

    /// Add the hotel to the map.
    fn add(&mut self, id: String, hotel: Hotel) {
        // add the hotel to the map if not yet present
        self.map.entry(id).or_insert(hotel);
    }
}

/// A struct for holding information about a single hotel
#[derive(Debug)]
struct Hotel {
    name: String,
    category: f32,
    city: String,
}

impl Hotel {
    /// Create a new Hotel from the HotelRow.
    fn new(row: HotelRow) -> Hotel {
        Hotel {
            name: row.name,
            category: row.category,
            city: row.city,
        }
    }

    /// Parse the string into Hotel and its id.
    fn parse(row: &str) -> (String, Hotel) {
        if let Ok(hotel_row) = serde_json::from_str::<HotelRow>(row) {
            (hotel_row.id.clone(), Hotel::new(hotel_row))
        } else {
            eprintln!("invalid hotel: {}", row);
            (
                String::from("???"),
                Hotel {
                    name: String::from("???"),
                    category: 0.0,
                    city: String::from("???"),
                },
            )
        }
    }
}

/// A struct that defines a single hotel row in a json file.
#[derive(Deserialize, PartialEq)]
struct HotelRow {
    id: String,
    city_code: String,
    name: String,
    category: f32,
    country_code: String,
    city: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An auxiliary function to create a HotelMap.
    #[test]
    fn create_hotel_map() {
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

        assert_eq!(2, result.map.len());
        assert!(result.map.contains_key("BER00002"));
        assert!(result.map.contains_key("BER00003"));
    }
}
