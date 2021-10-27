mod loader; // brings in everything in "loader.rs" as "loader" module
mod display;
pub mod configuration;
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

pub fn run(config: configuration::Config) {
    let (training, validation) = get_data("train.csv".to_string(), config.offset, config.count).unwrap();
    println!("Data load complete");

    let start = Instant::now();
    let mut errors = Vec::new();

    let (tx, rx) = mpsc::channel();
    for line in validation {
        let classifier = match &config.classifier[..] {
            "Manhattan Classifier" => recognize::get_manhattan_classifier(training.clone()),
            "Euclidean Classifier" => recognize::get_euclidean_classifier(training.clone()),
            _ => recognize::get_euclidean_classifier(training.clone()),
        };
    
        let tx = tx.clone();
        thread::spawn(move || {
            let (actual, predicted) = classifier.predict(&line);
            tx.send((actual, predicted)).unwrap();
        });
    }

    for _ in 0..config.count {
        let (actual, predicted) = rx.recv().unwrap();
        display_images(&actual, &predicted);

        if predicted.actual != actual.actual {
            errors.push((actual, predicted));
        }
    }

    let elapsed_time = start.elapsed().as_secs_f32();
    let total_errors = errors.len();

    print_summary(&config.classifier, config.offset, config.count, elapsed_time, total_errors);
    println!("Press <Enter> to show errors...");

    let mut discard = String::new();
    io::stdin().read_line(&mut discard).unwrap();

    println!("{}", "=".repeat(56));

    for (actual, predicted) in errors {
        display_images(&actual, &predicted);
    }

    print_summary(&config.classifier, config.offset, config.count, elapsed_time, total_errors);
    println!("DONE!");
}

pub fn print_summary(
    classifier: &str, 
    offset: usize, 
    count: usize, 
    elapsed_time: f32, 
    total_errors: usize) 
{
    println!("Using {} -- Offset: {}   Count: {}", classifier, offset, count);
    println!("Total time (seconds): {:.3}", elapsed_time);
    println!("Total errors: {}", total_errors);
}

pub fn get_data(filename: String, offset: usize, count: usize) -> io::Result<(Vec<Record>, Vec<Record>)> {
    let mut results = Vec::new();
    let contents = loader::get_raw_data(filename);
    for line in contents {
        let parsed = loader::parse_raw_data(&line);
        let rec = match loader::parse_record(parsed) {
            Ok(val) => val,
            Err(message) => {
                eprintln!("Bad record: {}", message);
                continue;
            },
        };
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
    println!("Actual: {} {} | Predicted: {}", actual.actual, " ".repeat(46), predicted.actual);
    let image = display::get_images_as_string(actual.image, predicted.image);
    print!("{}", image);
    println!("{}", "=".repeat(115));
}

