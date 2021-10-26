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

        let classifier: String = match &classifier[..] {
            "manhattan" => "Manhattan Classifier".to_string(),
            "euclidean" => "Euclidean Classifier".to_string(),
            _ => "Euclidean Classifier".to_string(),
        };

        Config {
            offset,
            count,
            classifier,
        }
    }
}