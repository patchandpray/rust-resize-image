// Tool to resize images to required width, keeping scale and then cropping to required height
extern crate image;

use std::env;
use std::fs::File;

use image::{FilterType, ImageOutputFormat};
use image::{GenericImageView, DynamicImage};

fn main() {

    // Get arguments from cmd line
    let args: Vec<String> = env::args().collect();

    let file_location = &args[1];
    let resize_width: u32 = args[2].parse().unwrap();
    let crop_height: u32 = args[3].parse().unwrap();
    let crop_method = &args[4];
    let output_file_location = &args[5];

    // open input image
    let img = image::open(&file_location).ok().expect("Opening image failed");

    // resize image according to width, keeping scale
    let mut resized_img = img.resize(resize_width, resize_width, FilterType::Lanczos3);

    // Set output image
    let mut out = File::create(output_file_location).unwrap();

    // Call crop function
    crop_image(crop_method, &mut resized_img, resize_width, crop_height, &mut out);

}

fn crop_image(crop_method: &str, resized_img: &mut DynamicImage, resize_width: u32, crop_height: u32, mut out: &mut File) {
    // determine crop method and calculate croppiness
    
    // Get resized image dimensions
    let (_new_width, new_height) = resized_img.dimensions();

    match crop_method.as_ref() {
        "bottom" => {
            let y: u32 = 0;
            let cropped_img = resized_img.crop(0, y, resize_width, crop_height);
            cropped_img.write_to(&mut out, ImageOutputFormat::PNG).unwrap();
        }
        "top"    => {
            let y: u32 = new_height - 281;
            let cropped_img = resized_img.crop(0, y, resize_width, crop_height);
            cropped_img.write_to(&mut out, ImageOutputFormat::PNG).unwrap();
        }
        "even"   => {
            let mut y: u32 = new_height - 281;
            y = y / 2;
            let cropped_img = resized_img.crop(0, y, resize_width, crop_height);
            cropped_img.write_to(&mut out, ImageOutputFormat::PNG).unwrap();
        }
        _ => panic!("not a valid crop method")
    }
}
