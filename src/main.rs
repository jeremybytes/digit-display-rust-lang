fn main() {
    let contents = digits::get_data("train.csv".to_string()).unwrap();
    let mut count = 0;
    for line in contents {
        if count > 10 {
            break;
        }
        println!("{:?}", line);
        count += 1;
    }
    
}
