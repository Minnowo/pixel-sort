use image::Rgb;
use image::RgbImage;

pub mod color;
pub mod interval;
pub mod sorting;

pub fn get_sorted_image(
    image: &RgbImage,
    mask_data: Option<&Vec<Vec<bool>>>,
    intervals: & Vec<Vec<u32>>,
    randomness: f32,
    sort_method: &sorting::SortMethod,
) -> RgbImage {

    let (width, height) = image.dimensions();

    let result = get_sorted_image_raw(image, mask_data, intervals, randomness, sort_method);

    let mut output = RgbImage::new(width, height);

    for (y, row) in result.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let x = x as u32;
            let y = y as u32;
            output.put_pixel(x, y, *pixel);
        }
    }
    output
}

pub fn sort_image(
    image: &mut RgbImage,
    mask_data: Option<&Vec<Vec<bool>>>,
    intervals: & Vec<Vec<u32>>,
    randomness: f32,
    sort_method: &sorting::SortMethod,
)  {
    let result = get_sorted_image_raw(image, mask_data, intervals, randomness, sort_method);

    for (y, row) in result.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let x = x as u32;
            let y = y as u32;
            image.put_pixel(x, y, *pixel);
        }
    }
}

pub fn get_sorted_image_raw(
    image: &RgbImage,
    mask_data: Option<&Vec<Vec<bool>>>,
    intervals: & Vec<Vec<u32>>,
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

pub fn create_bool_2d_vector(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut bool_2d_vector = Vec::with_capacity(height);
    for _ in 0..height {
        let row = vec![true; width];
        bool_2d_vector.push(row);
    }
    bool_2d_vector
}
