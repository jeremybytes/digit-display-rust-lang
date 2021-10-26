pub fn get_images_as_string(image1: [u8; 784], image2: [u8; 784]) -> String {
    let first_image = get_image_as_string(image1);
    let mut first = first_image.split("\n");
    let second_image = get_image_as_string(image2);
    let mut second = second_image.split("\n");
    let mut result = "".to_string();
    for _ in 0..28 {
        result += first.next().unwrap_or_default();
        result += " | ";
        result += second.next().unwrap_or_default();
        result += "\n"
    }
    result
}

pub fn get_image_as_string(image: [u8; 784]) -> String {
    let mut result = String::new();
    let mut count = 0;
    for pixel in image {
        if count % 28 == 0 && count != 0 {
            result += "\n";
        }
        let output_char = get_display_char_for_pixel(pixel);
        result += &output_char;
        result += &output_char;
        count += 1;
    }
    result += "\n";
    result
}

fn get_display_char_for_pixel(pixel: u8) -> String {
    match pixel {
        16..=31 => ".".to_string(),
        32..=63 => ":".to_string(),
        64..=159 => "o".to_string(),
        160..=223 => "O".to_string(),
        224..=255 => "@".to_string(),
        _ => " ".to_string(),
    }
}