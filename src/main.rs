use std::env;
use std::path::Path;

use image::GenericImageView;
use image::Rgb;
use image::RgbImage;
use rgb::RGB8;

pub mod color;

pub mod interval;

use color::HSB;

fn sort_image(
    image: &RgbImage,
    mask_data: &Vec<Vec<bool>>,
    intervals: &mut Vec<Vec<u32>>,
) -> Vec<Vec<Rgb<u8>>> {
    let mut sorted_pixels: Vec<Vec<Rgb<u8>>> = Vec::new();
    let mut interval_iter = intervals.iter();
    let (width, height) = image.dimensions();

    for y in 0..height {
        let mut row: Vec<Rgb<u8>> = Vec::new();

        let mut x_min = 0;

        if let Some(i) = interval_iter.next() {
            for x_max in i {
                let mut interval: Vec<Rgb<u8>> = Vec::new();

                for x in x_min..*x_max {
                    if mask_data[x as usize][y as usize] {
                        interval.push(image.get_pixel(x, y).clone());
                    }
                }

                if false {
                    // random.random() * 100 < randomness {
                    row.extend(interval);
                } else {
                    interval.sort_by(|a, b| {
                        let a_hue = HSB::rgb_get_hue(&a[0], &a[1], &a[2]);
                        let b_hue = HSB::rgb_get_hue(&b[0], &b[1], &b[2]);

                        a_hue.partial_cmp(&b_hue).unwrap()
                    });
                    row.extend(interval);
                }
            }
            sorted_pixels.push(row);
        } else {
            break;
        }
    }
    sorted_pixels
}


fn create_bool_2d_vector(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut bool_2d_vector = Vec::with_capacity(height);
    for _ in 0..height {
        let row = vec![false; width];
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

    let mut img;
    match image::open(image_path) {
        Ok(image) => img = image,
        Err(e) => {
            println!("There was an error reading the image: {}", e);
            std::process::exit(1)
        }
    }

    let (width, height) = img.dimensions();

    let mask_data = create_bool_2d_vector(width as usize, height as usize);

    let buffer = img.as_rgb8().unwrap();

    let mut intervals = interval::threshold(&buffer, 0.8, 0.8);

    let result = sort_image(&buffer,& mask_data, &mut intervals);

    let mut output = RgbImage::new(width, height);


    for (y, row) in result.iter().enumerate() {
        for(x, pixel) in row.iter().enumerate() {
            output.put_pixel(x as u32, y as u32, *pixel);
        }
    }

    let save_path = image_path.clone() + "_inverted.png";
    output.save(save_path).unwrap();
}
