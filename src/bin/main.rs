use lib::jpg::{Jpg, JpgOption};

const NOISE_CANCEL_COVER_COUNT: u32 = 2;
const NOISE_CANCEL_ITERATION: u32 = 4;

fn main() {
    let img = image::open("img/original.jpg").unwrap();
    let mut jpg = Jpg::new(img);
    let opt_gray = JpgOption::new(1.0, 1.0, 1.0, true, 1.0);
    let opt_green = JpgOption::new(0.0, 1.0, 0.0, false, 1.0);
    image::save_buffer(
        "img/gray.jpg",
        &jpg.export(&opt_gray),
        jpg.width,
        jpg.height,
        image::ColorType::Rgb8,
    )
    .unwrap();

    image::save_buffer(
        "img/green.jpg",
        &jpg.export(&opt_green),
        jpg.width,
        jpg.height,
        image::ColorType::Rgb8,
    )
    .unwrap();

    image::save_buffer(
        "img/bone.jpg",
        &jpg.export_bone(&opt_gray),
        jpg.width,
        jpg.height,
        image::ColorType::Rgb8,
    )
    .unwrap();

    jpg.remove_noise(NOISE_CANCEL_COVER_COUNT, NOISE_CANCEL_ITERATION);

    image::save_buffer(
        "img/bone_noise_canceled.jpg",
        &jpg.export_bone(&opt_gray),
        jpg.width,
        jpg.height,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
