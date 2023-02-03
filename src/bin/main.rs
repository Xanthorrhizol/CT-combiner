use lib::jpg::Jpg;
use lib::obj::Object;
use std::env;
use std::fs;
use std::process;

const NOISE_CANCEL_COVER_COUNT: u32 = 2;
const NOISE_CANCEL_ITERATION: u32 = 4;
const DOTS_PER_MM: u32 = 2; // TODO: get this data from CD
const Z_STEP: f32 = 2.0 * (DOTS_PER_MM as f32); // CT's distance btw each pictures is 2mm

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 4 {
        println!(
            "Usage: {} <pictures' dir> <output file> <use_boundary(y|n)>",
            args[0]
        );
        process::exit(-1);
    }
    let input_dir = &args[1];
    let output_file = &args[2];
    let use_boundary = match (&args[3]).to_string().as_str() {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => {
            println!(
                "Usage: {} <pictures' dir> <output file> <use_boundary(y|n)>",
                args[0]
            );
            process::exit(-1);
        }
    };
    let dir = fs::read_dir(input_dir).unwrap();
    let mut z = Box::new(0.0);
    let mut object = Box::new(Object::new());
    dir.for_each(|file| {
        let img = image::open(file.unwrap().path()).unwrap();
        let mut jpg = Jpg::new(img);
        jpg.remove_noise(NOISE_CANCEL_COVER_COUNT, NOISE_CANCEL_ITERATION);
        let coords = jpg.get_xy_coords(*z, use_boundary);
        (*object).stack_layer(coords);
        *z += Z_STEP;
    });
    (*object).export(output_file.to_owned());
}
