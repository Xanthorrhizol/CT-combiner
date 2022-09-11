use lib::jpg::{Jpg, JpgOption};
use std::env;
use std::process;

const NOISE_CANCEL_COVER_COUNT: u32 = 2;
const NOISE_CANCEL_ITERATION: u32 = 4;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pictures' path> <output file>", args[0]);
        process::exit(-1);
    }
    let input_path = &args[1];
    let output_file = &args[2];
    let img = image::open(input_path).unwrap();
    let mut jpg = Jpg::new(img);
    let opt = JpgOption::new(1.0, 1.0, 1.0, true, 1.0);
    jpg.remove_noise(NOISE_CANCEL_COVER_COUNT, NOISE_CANCEL_ITERATION);
    image::save_buffer(
        output_file,
        &jpg.export_boundary(&opt),
        jpg.width,
        jpg.height,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
