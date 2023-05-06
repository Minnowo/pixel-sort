use image::GenericImageView;
use std::path::Path;

use pixel_sort::interval;
use pixel_sort::sorting;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt)]
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

    #[structopt(short = "m", long = "interval", default_value = "random")]
    interval_method: String,

    #[structopt(short = "s", long = "sort", default_value = "hue")]
    sort_method: String,

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

    let sort_method= 
    match opt.sort_method.to_lowercase().as_str() {
        "hue" => sorting::SortMethod::Hue,
        "hsbsat" | "hsbsaturation" => sorting::SortMethod::HsbSaturation,
        "hslsat" | "hslsaturation" => sorting::SortMethod::HslSaturation,
        "light" | "lightness" => sorting::SortMethod::Lightness,
        "bright" | "brightness" => sorting::SortMethod::Brightness,
        "intensity" => sorting::SortMethod::Intensity,
        "min" | "minimum" => sorting::SortMethod::Minimum,
        _ =>  sorting::SortMethod::Hue,
        };

    let interval_by =
    match opt.interval_method.to_lowercase().as_str() {
        "rand" | "random" => interval::Interval::Random,
        "thresh" | "threshold" =>  interval::Interval::Threshold,
        "entire" | "row" | "full" =>  interval::Interval::EntireRow,
        _ =>  interval::Interval::Random,
    };

    let (width, height) = img.dimensions();
    println!(
        "Image loaded with size {}x{} type: {:?}",
        width,
        height,
        img.color()
    );

    let mut buffer = img.into_rgb8();

    let intervals = interval::get_interval(
        &interval_by,
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
        Err(e) => println!("{}", e),
    }
    println!("Image saved to {}", opt.output.to_string_lossy());
}
