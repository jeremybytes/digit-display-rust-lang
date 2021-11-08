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
    let (training, validation) = loader::get_data("train.csv".to_string(), config.offset, config.count).unwrap();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Data load complete");

    let start = Instant::now();
    let mut errors = Vec::new();

    let (tx, rx) = mpsc::channel();

    let chunks = loader::chunk_data(validation, config.threads);

    for chunk in chunks {
        let classifier = match &config.classifier[..] {
            "Manhattan Classifier" => recognize::get_manhattan_classifier(training.clone()),
            "Euclidean Classifier" => recognize::get_euclidean_classifier(training.clone()),
            _ => recognize::get_euclidean_classifier(training.clone()),
        };
        let tx = tx.clone();
        thread::spawn(move || {
            for line in chunk {
                let (actual, predicted) = classifier.predict(&line);
                tx.send((actual, predicted)).unwrap();
        }});
    }

    for _ in 0..config.count {
        let (actual, predicted) = rx.recv().unwrap();
        display_images(&actual, &predicted, false);

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
        display_images(&actual, &predicted, true);
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

pub fn display_image(data: &Record, scroll: bool) {
    if !scroll {
        print!("{esc}[1;1H", esc = 27 as char);
    }
    let image = display::get_image_as_string(data.image);
    print!("{}/n{}", image, "=".repeat(56));
}

pub fn display_images(actual: &Record, predicted: &Record, scroll: bool) {
    let image = display::get_images_as_string(actual.image, predicted.image);
    if !scroll {
        print!("{esc}[1;1H", esc = 27 as char);
    }
    println!("Actual: {} {} | Predicted: {}\n{}\n{}", actual.actual, " ".repeat(46), predicted.actual, image, "=".repeat(115));
}

