use std::io;

#[derive(Debug)]
pub struct Record {
    pub actual: u8,
    pub image: [u8; 784],
}

use self::loading::*;

pub fn get_data(filename: String) -> io::Result<Vec<Record>> {
    let mut results = Vec::new();
    let contents = get_raw_data(filename);
    for line in contents {
        let parsed = parse_raw_data(&line).clone();
        let rec = parse_record(parsed);
        results.push(rec);
    }
    Ok(results)
}

pub mod loading {
    use std::fs;
    
    pub fn get_raw_data(filename: String) -> Vec<String> {
        let contents = fs::read_to_string(filename).unwrap();
        let splits: Vec<&str> = contents.split("\r\n").skip(1).collect();
        let mut results = Vec::new();
        for split in splits {
            results.push(split.clone().to_string())
        }
        results
    }

    pub fn parse_raw_data(raw_data: &str) -> Vec<u8> {
        let mut results = Vec::new();
        let items: Vec<&str> = raw_data.split(',').collect();
        for item in items
        {
            let i: u8 = match item.trim().parse() {
                Ok(value) => value,
                Err(_) => continue,
            };
            results.push(i);
        }
        results
    }

    pub fn parse_record(data: Vec<u8>) -> super::Record {
        let mut iterator = data.into_iter();
        let actual = iterator.next().unwrap_or_default();
        let mut image: [u8; 784] = [0; 784];
        let mut index = 0;
        for i in iterator {
            image[index] = i;
            index += 1;
        }
        super::Record {
            actual,
            image,
        }
    }
}