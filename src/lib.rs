mod loader; // brings in everything in "loader.rs" as "loader" module
mod display;
pub mod recognize;

use std::io;

#[derive(Debug,Clone)]
pub struct Record {
    pub actual: u8,
    pub image: [u8; 784],
}

pub fn get_data(filename: String) -> io::Result<(Vec<Record>, Vec<Record>)> {
    let mut results = Vec::new();
    let contents = loader::get_raw_data(filename);
    for line in contents {
        let parsed = loader::parse_raw_data(&line).clone();
        let rec = loader::parse_record(parsed);
        results.push(rec);
    }
    let data_sets = loader::split_data_sets(results, 1000, 100);

    Ok(data_sets)
}

pub fn display_image(data: Record) {
    let image = display::get_image_as_string(data.image);
    println!("Actual: {}", data.actual);
    print!("{}", image);
    println!("{}", "=".repeat(56));
}

