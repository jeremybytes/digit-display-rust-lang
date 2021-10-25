pub struct Classifier {
    pub name: String,
    training_data: Vec<super::Record>,
    algorithm: fn(input: u8, test: u8) -> i64,
}

impl Classifier {
    pub fn predict<'a>(&self, input: &'a super::Record) -> (&'a super::Record, &super::Record) {
        let mut best_total = 100000000;
        let mut best = &super::Record {actual: 0, image: [0; 784]};
        for item in self.training_data.iter() {
            let mut total: i64 = 0;
            for i in 0..783 {
                let diff = (self.algorithm)(input.image[i], item.image[i]);
                total += diff;
            }
            if total < best_total {
                best_total = total;
                best = item;
            }
        }
    
        (input, &best)    
    }
}

pub fn get_manhattan_classifier(training_data: Vec<super::Record>) -> Classifier {
    Classifier {
        name: "Manhattan Classifier".to_string(),
        training_data,
        algorithm: manhattan_algorithm,
    }
}

fn manhattan_algorithm(input: u8, test: u8) -> i64 {
    ((i64::from(input))-(i64::from(test))).abs()
}

pub fn get_euclidean_classifier(training_data: Vec<super::Record>) -> Classifier {
    Classifier {
        name: "Euclidean Classifier".to_string(),
        training_data,
        algorithm: euclidean_algorithm,
    }
}

fn euclidean_algorithm(input: u8, test: u8) -> i64 {
    let diff = (i64::from(input))-(i64::from(test));
    diff * diff
}