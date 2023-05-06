use image::GenericImageView;
use std::path::Path;

use pixel_sort::interval;
use pixel_sort::sorting;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    #[structopt(short = "r", long = "randomness", default_value = "0")]
    randomness: f32,

    #[structopt(short = "L", long = "length", default_value = "50")]
    interval_length: u32,

    #[structopt(short = "l", long = "lower_threshold", default_value = "0.2")]
    lower_threshold: f32,

    #[structopt(short = "u", long = "upper_threshold", default_value = "0.8")]
    upper_threshold: f32,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}




fn main() {
    let opt = Opt::from_args();

    if !Path::new(&opt.input).is_file() {
        println!(
            "The given path {} does not exist!",
            opt.input.to_string_lossy()
        );
        std::process::exit(1)
    }

    let img;
    match image::open(opt.input) {
        Ok(image) => img = image,
        Err(e) => {
            println!("There was an error reading the image: {}", e);
            std::process::exit(1)
        }
    }

    let sort_method = sorting::SortMethod::HsbSaturation;
    let inverval_by = interval::Interval::Random;

    let (width, height) = img.dimensions();
    println!(
        "Image loaded with size {}x{} type: {:?}",
        width,
        height,
        img.color()
    );

    let mut buffer = img.into_rgb8();

    let intervals = interval::get_interval(
        &inverval_by,
        &buffer,
        &opt.interval_length,
        &opt.lower_threshold,
        &opt.upper_threshold,
    );

    println!("Intervals found!");
    println!("Starting sorting...");

    pixel_sort::sort_image(
        &mut buffer,
        Option::None,
        &intervals,
        opt.randomness,
        &sort_method,
    );
    println!("Sorting done!");
    println!("Saving image...");

    match buffer.save(&opt.output) {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }
    println!("Image saved to {}", opt.output.to_string_lossy());
}
