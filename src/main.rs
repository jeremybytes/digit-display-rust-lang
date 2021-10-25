use std::env;

use digits::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    digits::run(config);
}
