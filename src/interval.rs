
use image::RgbImage;

pub enum Interval {
    Threshold,
    Random,
    EntireRow,
}
pub fn get_interval(interval_method : &Interval, image: &RgbImage, char_length:& u32, lower_threshold: &f32, upper_threshold: &f32) -> Vec<Vec<u32>>{
    match interval_method {
        Interval::Threshold => threshold(image, lower_threshold, upper_threshold),
        Interval::Random => random(image, char_length),
        Interval::EntireRow => entire_row(image),
    }
}
pub fn entire_row(image : &RgbImage) -> Vec<Vec<u32>> {

    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    for _y in 0..height {

        let row:Vec<u32> = Vec::from([0, width ]);
        intervals.push(row);
    }

    return intervals;
}

pub fn random(image: &RgbImage, char_length:& u32) -> Vec<Vec<u32>> {


    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    let mut char_length = *char_length;

    if char_length < 1 {
       char_length = 2; 
    }

    for _y in 0..height {

        let mut row:Vec<u32> = Vec::new();

        let mut x = 0;

        loop {

            x += (char_length as f32 * rand::random::<f32>()) as u32;

            if x > width {
                break; 
            }
            else {
                row.push(x);
            }
        }
        if x < width  {
            row.push(width);
        }
        intervals.push(row);
    }

    return intervals;

}

pub fn threshold(image: &RgbImage, lower_threshold: &f32, upper_threshold: &f32) -> Vec<Vec<u32>> {

    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    for y in 0..height {

        let mut row:Vec<u32> = Vec::new();
       
        for x in 0..width {

            let pixel: &image::Rgb<u8> = image.get_pixel(x, y);
            let level = crate::color::hsb::rgb_get_brightness(&pixel[0],&pixel[1], &pixel[2]);
            
            if level < *lower_threshold  || level > *upper_threshold  {
                row.push(x);
            }
        }
        if row.last().unwrap()< &width {
            row.push(width);
        }
        intervals.push(row);
    }

    return intervals;

}




