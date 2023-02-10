use image::{DynamicImage,GenericImageView};
use std::env;

fn convert_to_ascii(img: &DynamicImage) -> String {
    let (width,height) = img.dimensions();
    let mut ascii_img = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            // Value of R,G,B
            let avg_brightness = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64 ) / 3.0; 
            // Get characters according to brightness
            let ascii_char = get_ascii_char(avg_brightness);
            ascii_img.push(ascii_char);
        }
        ascii_img.push('\n');
    }

    ascii_img
}

fn get_ascii_char(brightness: f64) -> char {
    let ascii_chars = " .,-~+=@";
        //" .`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCL0QOZmwqpdbkhao*#WM&8%B@$";
    
    // Divide the range into length of ascii_chars
    let index = (brightness / 255.0 * (ascii_chars.len() as f64 - 1.0)) as usize;
    ascii_chars.chars().nth(index).unwrap()
}

fn main() {
    let args = env::args().nth(1).unwrap_or_else(|| "pug.png".to_string() );
    // Load the image
    let image = image::open(args).unwrap();
    // Resize the image
    let resized_image = image.resize_exact(80, 40, image::imageops::FilterType::CatmullRom);

    let ascii_img = convert_to_ascii(&resized_image);

    println!("{ascii_img}");
}
