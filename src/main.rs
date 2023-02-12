use image::{DynamicImage,GenericImageView};
use std::env;
use colored::*;

fn convert_to_ascii(img: &DynamicImage, colored: bool) -> String {
    let (width,height) = img.dimensions();
    let mut ascii_img = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            // Value of R,G,B
            let avg_brightness = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64 ) / 3.0; 
            // Get characters according to brightness
            let ascii_char = get_ascii_char(avg_brightness,colored);
            if colored {
                let mut ch = format!("{ascii_char}");
                ch = format!("{}",ch.truecolor(pixel[0],pixel[1], pixel[2]) );
                ascii_img.push_str(&ch);

            } else {
                ascii_img.push(ascii_char );
            }
        }
        ascii_img.push('\n');
    }

    ascii_img
}

fn get_ascii_char(brightness: f64,colored: bool) -> char {
    let ascii_chars = if !colored {
        " .,-~+=@"
    } else {
        " .`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCL0QOZmwqpdbkhao*#WM&8%B@$"
    };
    
    // Divide the range into length of ascii_chars
    let index = (brightness / 255.0 * (ascii_chars.len() as f64 - 1.0)) as usize;
    ascii_chars.chars().nth(index as usize).unwrap()
}

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: cargo run [.jpg/.png] [any character to print colored]");
        return;
    }
    args.next();
    let image = args.next().unwrap();
    let colored = args.next().is_some();

    // Load the image
    let image = image::open(image).unwrap();
    // Resize the image
    let resized_image = image.resize_exact(80, 40, image::imageops::FilterType::CatmullRom);

    let ascii_img = convert_to_ascii(&resized_image, colored);

    println!("{ascii_img}");
}
