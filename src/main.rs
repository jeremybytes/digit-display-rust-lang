fn main() {
    let (_training, validation) = digits::get_data("train.csv".to_string()).unwrap();
    let mut count = 0;
    for line in validation {
        if count > 10 {
            break;
        }
        digits::display_image(line);
        count += 1;
    }
    
}
