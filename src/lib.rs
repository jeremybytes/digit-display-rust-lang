mod loading; // brings in everything in "loading.rs" as "loading" module
mod display;

use std::io;

#[derive(Debug)]
pub struct Record {
    pub actual: u8,
    pub image: [u8; 784],
}

pub fn get_data(filename: String) -> io::Result<Vec<Record>> {
    let mut results = Vec::new();
    let contents = loading::get_raw_data(filename);
    for line in contents {
        let parsed = loading::parse_raw_data(&line).clone();
        let rec = loading::parse_record(parsed);
        results.push(rec);
    }
    Ok(results)
}

pub fn display_image(data: Record) {
    let image = display::get_image_as_string(data.image);
    println!("Actual: {}", data.actual);
    print!("{}", image);
    println!("{}", "=".repeat(56));
}
