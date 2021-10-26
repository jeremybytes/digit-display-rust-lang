mod loader; // brings in everything in "loader.rs" as "loader" module
mod display;
pub mod recognize;

use std::io;
use std::time::Instant;
use std::thread;
use std::sync::mpsc;

#[derive(Debug,Clone)]
pub struct Record {
    pub actual: u8,
    pub image: [u8; 784],
}

pub struct Config {
    pub offset: usize,
    pub count: usize,
    pub classifier: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let mut offset = "".to_string();
        let mut count = "".to_string();
        let mut classifier = "".to_string();
        if args.len() > 1 {
            offset = args[1].clone();
        }
        if args.len() > 2 {
            count = args[2].clone();
        }
        if args.len() > 3 {
            classifier = args[3].clone().to_lowercase();
        }

        let offset: usize = match offset.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid offset, using default (1000)");
                1000
            },
        };

        let count: usize = match count.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid count, using default (100)");
                100
            },
        };
        Config {
            offset,
            count,
            classifier,
        }
    }
}

pub fn run(config: Config) {
    let (training, validation) = get_data("train.csv".to_string(), config.offset, config.count).unwrap();

    println!("Data load complete");

    let start = Instant::now();
    let mut errors = Vec::new();

    let (tx, rx) = mpsc::channel();
    for line in validation {
        let classifier = match &config.classifier[..] {
            "manhattan" => recognize::get_manhattan_classifier(training.clone()),
            "euclidean" => recognize::get_euclidean_classifier(training.clone()),
            _ => recognize::get_euclidean_classifier(training.clone()),
        };
    
        let test_item = line.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let (actual, predicted) = classifier.predict(&test_item);
            tx.send((actual.clone(), predicted.clone())).unwrap();
        });
    }

    for _ in 0..config.count {
        let (actual, predicted) = rx.recv().unwrap();
        if predicted.actual != actual.actual {
            errors.push((actual.clone(), predicted.clone()));
        }

        println!("Actual: {} {} | Predicted: {}", actual.actual, " ".repeat(46), predicted.actual);
        display_images(&actual, &predicted);
    }

    let total_seconds = start.elapsed().as_secs();
    
    let total_errors = errors.len();
    println!("Using {} -- Offset: {}   Count: {}", &config.classifier, config.offset, config.count);
    println!("Total time (seconds): {}", total_seconds);
    println!("Total errors: {}", total_errors);

    println!("Press <Enter> to show errors...");
    let mut discard = String::new();
    io::stdin().read_line(&mut discard).unwrap();

    println!("{}", "=".repeat(56));
    for (actual, predicted) in errors {
        println!("Actual: {} {} | Predicted: {}", actual.actual, " ".repeat(46), predicted.actual);
        display_images(&actual, &predicted);
    }
    println!("Using {} -- Offset: {}   Count: {}", &config.classifier, config.offset, config.count);
    println!("Total time (seconds): {}", total_seconds);
    println!("Total errors: {}", total_errors);
    println!("DONE!");
}

pub fn get_data(filename: String, offset: usize, count: usize) -> io::Result<(Vec<Record>, Vec<Record>)> {
    let mut results = Vec::new();
    let contents = loader::get_raw_data(filename);
    for line in contents {
        let parsed = loader::parse_raw_data(&line).clone();
        let rec = loader::parse_record(parsed);
        results.push(rec);
    }
    let data_sets = loader::split_data_sets(results, offset, count);

    Ok(data_sets)
}

pub fn display_image(data: &Record) {
    let image = display::get_image_as_string(data.image);
    print!("{}", image);
    println!("{}", "=".repeat(56));
}

pub fn display_images(actual: &Record, predicted: &Record) {
    let image = display::get_images_as_string(actual.image, predicted.image);
    print!("{}", image);
    println!("{}", "=".repeat(115));
}

