
use image::{RgbImage, GenericImageView};

use crate::color::HSB;

pub enum Interval {
    Threshold,
    Random
}



pub fn threshold(image: &RgbImage, lower_threshold: f32, upper_threshold: f32) -> Vec<Vec<u32>> {

    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    for y in 0..height {

        let mut row:Vec<u32> = Vec::new();

        for x in 0..width {

            let pixel: &image::Rgb<u8> = image.get_pixel(x, y);
            let level = HSB::rgb_get_brightness(&pixel[0],&pixel[1], &pixel[2]);

            if level < lower_threshold * 255f32 || level > upper_threshold * 255f32 {
                row.push(x);
            }
        }
    }

    return intervals;

}




