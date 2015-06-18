extern crate env_logger;
extern crate image;
extern crate time;

use cast::From;
use self::image::{ImageBuffer, Luma};

use ::{Col, Mat};

const RGB_PATH: &'static str = "rgb.jpg";
const GRAY_PATH: &'static str = "gray.jpg";

pub fn main() {
    env_logger::init().unwrap();

    let rgb = image::open(RGB_PATH).unwrap();

    let start = time::precise_time_ns();
    let _ = rgb.to_luma();
    let end = time::precise_time_ns();

    let rgb = rgb.to_rgb();

    let (width, height) = rgb.dimensions();
    let npixels = width * height;

    println!("Converting a {}x{} RGB image to grayscale", width, height);

    println!("using `image::DynamicImage::to_luma`: {} ns", end - start);

    // Split in color channels
    let rgb = Mat::reshape(&rgb, (npixels, 3));
    let r = &rgb[.., 0];
    let g = &rgb[.., 1];
    let b = &rgb[.., 2];
    // NB Only allocation required for this transformation
    let mut gray = Col::zeros(npixels);

    // Apply RGB -> grayscale transform
    let start = time::precise_time_ns();
    gray[..] = {
        r.map(f32::from) * 0.2126 +
        g.map(f32::from) * 0.7152 +
        b.map(f32::from) * 0.0722
    }.map(|x| x as u8);
    let end = time::precise_time_ns();

    println!("using expression templates: {} ns", end - start);

    ImageBuffer::<Luma<u8>, _>::from_raw(width, height, gray.as_ref())
        .unwrap()
        .save(GRAY_PATH)
        .unwrap();
}
