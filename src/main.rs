use std::env;
use std::path::Path;

use image::GenericImageView;
use image::Rgb;
use image::RgbImage;

mod color;
mod interval;
mod sorting;

fn sort_image(
    image: &RgbImage,
    mask_data: Option<&Vec<Vec<bool>>>,
    intervals: &mut Vec<Vec<u32>>,
    randomness: f32,
    sort_method: &sorting::SortMethod,
) -> Vec<Vec<Rgb<u8>>> {
    let mut sorted_pixels: Vec<Vec<Rgb<u8>>> = Vec::new();
    let mut interval_iter = intervals.iter();
    let (_width, height) = image.dimensions();

    let mut last_progress = 0;

    for y in 0..height {
        let mut row: Vec<Rgb<u8>> = Vec::new();

        let mut x_min = 0;

        if let Some(i) = interval_iter.next() {
            for x_max in i {
                let mut interval: Vec<Rgb<u8>> = Vec::new();

                for x in x_min..*x_max {
                    if let Some(mask) = mask_data {
                        if mask[x as usize][y as usize] {
                            interval.push(image.get_pixel(x, y).clone());
                        }
                    } else {
                        interval.push(image.get_pixel(x, y).clone());
                    }
                }

                if randomness > 0f32 && rand::random::<f32>() * 100f32 < randomness {
                    row.extend(interval);
                } else {
                    interval.sort_by(sorting::get_sort_func(sort_method));
                    row.extend(interval);
                }

                x_min = *x_max;
            }

            let progress = ((y + 1) as f32 / height as f32 * 100f32) as u32;

            if progress % 10 == 0 && progress != last_progress {
                println!("Sort progress: {}%", progress);
            }
            last_progress = progress;
            sorted_pixels.push(row);
        } else {
            println!("Early break for some reason!!!");
            break;
        }
    }
    sorted_pixels
}

fn create_bool_2d_vector(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut bool_2d_vector = Vec::with_capacity(height);
    for _ in 0..height {
        let row = vec![true; width];
        bool_2d_vector.push(row);
    }
    bool_2d_vector
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide path to an image file!");
        std::process::exit(1)
    }

    let image_path = &args[1];

    if !Path::new(&image_path).is_file() {
        println!("The given path {} does not exist!", image_path);
        std::process::exit(1)
    }

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.

    let img;
    match image::open(image_path) {
        Ok(image) => img = image,
        Err(e) => {
            println!("There was an error reading the image: {}", e);
            std::process::exit(1)
        }
    }

    let lower_shresh = 0.1;
    let upper_shresh = 0.9;
    let random_amount = 50;
    let randomness = 0f32;
    let sortBy = sorting::SortMethod::Intensity;

    let (width, height) = img.dimensions();
    println!("Image loaded with size {}x{}", width, height);
    let mask_data = Option::None; //create_bool_2d_vector(width as usize, height as usize);
    println!("mask_data created!");
    let buffer = img.as_rgb8().unwrap();

    let mut intervals = interval::get_interval(
        &interval::Interval::EntireRow,
        &buffer,
        &random_amount,
        &lower_shresh,
        &upper_shresh,
    );

    println!("Intervals found!");
    println!("Starting sorting...");
    let result = sort_image(&buffer, mask_data, &mut intervals, randomness, &sortBy);

    let mut output = RgbImage::new(width, height);

    println!("Sorting done!");
    println!("Building output image...");

    for (y, row) in result.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let x = x as u32;
            let y = y as u32;
            if x >= width || y >= height {
                continue;
            }
            output.put_pixel(x, y, *pixel);
        }
    }

    println!("Output image built!");
    println!("Saving image...");
    let save_path = image_path.clone() + "_inverted.png";
    output.save(&save_path).unwrap();
    println!("Image saved to {}", save_path);
}
