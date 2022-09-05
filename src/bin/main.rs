use lib::jpg::Jpg;
use lib::obj::Object;
use std::env;
use std::fs;
use std::process;

const NOISE_CANCEL_COVER_COUNT: u32 = 2;
const NOISE_CANCEL_ITERATION: u32 = 4;
const STEP: f32 = 4.0;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pictures' dir> <output file>", args[0]);
        process::exit(-1);
    }
    let input_dir = &args[1];
    let output_file = &args[2];
    let dir = fs::read_dir(input_dir).unwrap();
    let mut z = Box::new(0.0);
    let mut object = Box::new(Object::new());
    dir.for_each(|file| {
        let img = image::open(file.unwrap().path()).unwrap();
        let mut jpg = Jpg::new(img);
        jpg.remove_noise(NOISE_CANCEL_COVER_COUNT, NOISE_CANCEL_ITERATION);
        let coords = jpg.get_xy_coords(*z);
        (*object).stack_layer(coords);
        *z += STEP;
    });
    (*object).export(output_file.to_owned());
}
