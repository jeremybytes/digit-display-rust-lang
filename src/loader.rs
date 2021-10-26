use std::fs;
    
pub fn get_raw_data(filename: String) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let splits: Vec<&str> = contents.split("\r\n").skip(1).collect();
    let mut results = Vec::new();
    for split in splits {
        results.push(split.to_string())
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

pub fn split_data_sets(data: Vec<super::Record>, offset: usize, count: usize) -> (Vec<super::Record>, Vec<super::Record>) {
    let training = [&data[..offset], &data[(offset+count)..]].concat();
    let validation = [&data[offset..(offset+count)]].concat();
    (training, validation)
}