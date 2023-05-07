
use image::RgbImage;

use crate::math;

pub enum IntervalType {
    HorizontalRow(Vec<Vec<u32>>),
    DynamicLine(Vec<Vec<(u32, u32)>>),
}

pub enum Interval {
    Threshold,
    Random,
    EntireRow,
    AbsSinWave,
    Angle
}
pub fn get_interval(
    interval_method: &Interval,
    image: &RgbImage,
    char_length: &u32,
    lower_threshold: &f32,
    upper_threshold: &f32,
    angle: &u32,
    threshold_inclusive: &bool,
) -> IntervalType {
    match interval_method {
        Interval::Threshold => IntervalType::HorizontalRow(threshold(
            image,
            lower_threshold,
            upper_threshold,
            threshold_inclusive,
        )),
        Interval::Random => IntervalType::HorizontalRow(random(image, char_length)),
        Interval::EntireRow => IntervalType::HorizontalRow(entire_row(image)),
        Interval::AbsSinWave => IntervalType::DynamicLine(sin_wave(image, char_length)),
        Interval::Angle => IntervalType::DynamicLine(intervals_from_angle(image, angle)),
    }
}
pub fn entire_row(image: &RgbImage) -> Vec<Vec<u32>> {
    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    for _y in 0..height {
        let row: Vec<u32> = Vec::from([0, width]);
        intervals.push(row);
    }

    return intervals;
}

pub fn random(image: &RgbImage, char_length: &u32) -> Vec<Vec<u32>> {
    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    let mut char_length = *char_length;

    if char_length < 1 {
        char_length = 2;
    }

    for _y in 0..height {
        let mut row: Vec<u32> = Vec::new();

        let mut x = 0;

        loop {
            x += (char_length as f32 * rand::random::<f32>()) as u32;

            if x > width {
                break;
            } else {
                row.push(x);
            }
        }
        if x < width {
            row.push(width);
        }
        intervals.push(row);
    }

    return intervals;
}

pub fn threshold(
    image: &RgbImage,
    lower_threshold: &f32,
    upper_threshold: &f32,
    inclusive: &bool,
) -> Vec<Vec<u32>> {
    let mut intervals: Vec<Vec<u32>> = Vec::new();

    let (width, height) = image.dimensions();

    for y in 0..height {
        let mut row: Vec<u32> = Vec::new();

        for x in 0..width {
            let pixel: &image::Rgb<u8> = image.get_pixel(x, y);
            let level = crate::color::hsl::rgb_get_lightness(&pixel[0], &pixel[1], &pixel[2]);

            if !*inclusive {
                if level > *lower_threshold && level < *upper_threshold {
                    row.push(x);
                }
            } else {
                if level < *lower_threshold || level > *upper_threshold {
                    row.push(x);
                }
            }
        }
        if !row.is_empty() && row.last().unwrap() < &width {
            row.push(width);
        }
        intervals.push(row);
    }

    return intervals;
}

pub fn extend_dynamic_line_interval_to_width_2(
    width: &u32,
    height: &u32,
    line: &Vec<(u32, u32)>,
) -> Vec<Vec<(u32, u32)>> {

    let mut intervals: Vec<Vec<(u32, u32)>> = Vec::new();

    if line.len() < 1 {
        return intervals;
    }

    for _ in 0..*width {
        intervals.push(Vec::new());
    }
    
    let mut line_iter = line.iter();
    let mut start = line_iter.next().unwrap();

    for next  in line_iter {

        let dist_x = (next.0 as i32 - start.0 as i32).abs() as u32;
        let dist_y = (next.1 as i32 - start.1 as i32).abs() as u32;

        let angle = (dist_y as f64 / dist_x as f64).atan();

        for (i, interval ) in (0..(*width )).step_by(1).zip(intervals.iter_mut()) {

            let y = (i as f64 * angle.tan()) as u32 ;

            let x = if dist_x > i {
                0
            }
             else {
                i - dist_x
             };

            interval.push((x, match next.1.checked_sub(y) {
                Some(y) => y,
                None => 0,
             }));
            interval.push((i, next.1));
        }

        start = next;
    }


intervals
}
pub fn extend_dynamic_line_interval_to_width(
    width: &u32,
    height: &u32,
    line: &Vec<(u32, u32)>,
) -> Vec<Vec<(u32, u32)>> {
    let mut intervals: Vec<Vec<(u32, u32)>> = Vec::new();

    for i in 0..*width {
        let mut t2 = Vec::new();

        line.iter().for_each(|p| {
            if p.0 + i < *width {
                t2.push((p.0 + i, p.1));
            } 
            // else {

            //     let h = if (p.1 as i32 - (p.0 as i32 + i as i32 - *width as i32) < 0) {
            //         0
            //     } else {
            //         p.1 - (p.0 + i - width)
            //     };

            //     t2.push((width - 1, h));
            // }
            
            
            if p.0 as i32 - i as i32 > 0 {
                t2.push((p.0 - i, p.1));
            } 
        });

        intervals.push(t2);
    }

    

    intervals
}
pub fn sin_wave(image: &RgbImage, char_length: &u32) -> Vec<Vec<(u32, u32)>> {
    let mut intervals = Vec::new();

    let (width, height) = image.dimensions();

    let mut y = 0;
    let mut x: f64 = 0_f64;

    while y < height {
        intervals.push((((x).sin().abs() * 100_f64) as u32, y));

        x += std::f64::consts::PI / 4_f64;
        y += char_length;
    }

    extend_dynamic_line_interval_to_width(&width,&height, &intervals)
}

pub fn intervals_from_angle(image: &RgbImage, angle: &u32) -> Vec<Vec<(u32, u32)>> {

    let (width, height) = image.dimensions();

    let angle = *angle % 180;

    if angle <= 90 {

        let radians = math::deg_to_rad(angle  as f64); 

        let hyp =  height as f64 / radians.sin();

        let mut x = radians.cos() * hyp;
        let mut y = height as f64 - 1_f64;

        // if x as u32 > width {
        //     let hyp = width as f64 / radians.cos();

        //     x = width as f64 - 1_f64;
        //     y = hyp * radians.sin();
        // }

        let start = (0_u32, 0_u32);
        let end = (x as u32, y as u32);

        println!("{} {}    {} {}", 0, 0, end.0, end.1);

        let v = vec![start, end];

        return extend_dynamic_line_interval_to_width_2(&width, &height, &v);
    }

    vec![]

}
