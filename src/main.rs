use std::io;
use std::time::{Instant};

fn main() {
    let (training, validation) = digits::get_data("train.csv".to_string()).unwrap();
    let mut count = 0;
    //let classifier = digits::recognize::get_manhattan_classifier(training);
    let classifier = digits::recognize::get_euclidean_classifier(training);

    let start = Instant::now();
    let mut errors = Vec::new();
    for line in validation {
        if count > 50 {
            break;
        }

        let (actual, predicted) = classifier.predict(&line);

        if predicted.actual != actual.actual {
            errors.push((actual.clone(), predicted.clone()));
        }

        println!("Predicted: {}   --   Actual: {}", predicted.actual, actual.actual);
        digits::display_image(line);

        count += 1;
    }
    let total_seconds = start.elapsed().as_secs();
    
    let total_errors = errors.len();
    println!("Total time (seconds): {}", total_seconds);
    println!("Total errors: {}", total_errors);
    println!("Press <Enter> to show errors...");
    let mut discard = String::new();
    io::stdin().read_line(&mut discard).unwrap();
    println!("{}", "=".repeat(56));
    for (actual, predicted) in errors {
        println!("Predicted: {}   --   Actual: {}", predicted.actual, actual.actual);
        digits::display_image(actual);
    }
    println!("Total time (seconds): {}", total_seconds);
    println!("Total errors: {}", total_errors);
    println!("DONE!");
}
