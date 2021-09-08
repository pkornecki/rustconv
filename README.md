# rustconv

rustconv is a program written in Rust, that converts input file(`input.csv`) into output file(`output.csv`).
Some data which is necessary for the converion is taken from the `hotel.json` file.

Some concepts used:
- csv/json serialization/deserialization
- asynchronous processing
- command line arguments parsing
- modularization
- error handling
- and more

## Input format: 

city_code|hotel_code|room_type|room_code|meal|checkin|adults|children|price|source

## Output format:

room_type&nbsp;meal;room_code;source;hotel_name;city_name;city_code;hotel_category;pax;adults;children;checkin;checkout;price

## hotels.json format:

Each line contains a single JSON object.

{
  "id":
  "city_code":
  "name":
  "category":
  "country_code":
  "city":
}

