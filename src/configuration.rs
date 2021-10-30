extern crate clap;

use clap::{Arg, App};


pub struct Config {
    pub offset: usize,
    pub count: usize,
    pub classifier: String,
    pub threads: usize,
}

impl Config {
    pub fn new() -> Config {

        let app = App::new("digits")
            .version("1.0")
            .about("parses hand-written digits")
            .author("Jeremy Clark");

        let offset_option = Arg::with_name("offset")
            .short("o")
            .long("offset") 
            .takes_value(true)
            .help("Offset in the data set (default: 1000)")
            .required(false);
        let app = app.arg(offset_option);

        let count_option = Arg::with_name("count")
            .short("c")
            .long("count") 
            .takes_value(true)
            .help("Number of records to process (default: 100)")
            .required(false);
        let app = app.arg(count_option);

        let classifier_option = Arg::with_name("classifier")
            .long("classifier") 
            .takes_value(true)
            .help("Classifier to use (default: 'euclidean')")
            .required(false);
        let app = app.arg(classifier_option);

        let threads_option = Arg::with_name("threads")
            .short("t")
            .long("threads") 
            .takes_value(true)
            .help("Number of threads to use (default: 6)")
            .required(false);
        let app = app.arg(threads_option);

        let matches = app.get_matches();

        let offset = matches.value_of("offset").unwrap_or_default();
        let count = matches.value_of("count").unwrap_or_default();
        let classifier = matches.value_of("classifier").unwrap_or_default();
        let threads = matches.value_of("threads").unwrap_or_default();

        let offset: usize = match offset.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Offset: using default (1000)");
                1000
            },
        };

        let count: usize = match count.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Count: using default (100)");
                100
            },
        };

        let classifier: String = match &classifier[..] {
            "manhattan" => "Manhattan Classifier".to_string(),
            "euclidean" => "Euclidean Classifier".to_string(),
            _ => "Euclidean Classifier".to_string(),
        };

        let threads: usize = match threads.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Threads: using default (6)");
                6
            },
        };

        Config {
            offset,
            count,
            classifier,
            threads,
        }
    }
}