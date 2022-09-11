const DAMP: u8 = 15;

pub struct JpgOption {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub gray_scale: bool,
    pub brightness: f32,
}
impl JpgOption {
    pub fn new(red: f32, green: f32, blue: f32, gray_scale: bool, brightness: f32) -> JpgOption {
        JpgOption {
            red,
            green,
            blue,
            gray_scale,
            brightness,
        }
    }
}

pub struct Jpg {
    pub width: u32,
    pub height: u32,
    pub red: Vec<u8>,
    pub green: Vec<u8>,
    pub blue: Vec<u8>,
    pub light: Vec<u8>,
    pub binary: Vec<bool>,
    pub boundary: Vec<bool>,
}
impl Jpg {
    pub fn new(img: image::DynamicImage) -> Jpg {
        let width = img.width();
        let height = img.height();
        let mut red: Vec<u8> = vec![];
        let mut green: Vec<u8> = vec![];
        let mut blue: Vec<u8> = vec![];
        let mut light: Vec<u8> = vec![];
        let mut i = 0;
        for val in img.into_rgb8().iter() {
            match i % 3 {
                0 => {
                    red.push(val.to_owned().into());
                }
                1 => {
                    green.push(val.to_owned().into());
                }
                2 => {
                    blue.push(val.to_owned().into());
                    let sum: u32 = red.last().unwrap().to_owned() as u32
                        + green.last().unwrap().to_owned() as u32
                        + blue.last().unwrap().to_owned() as u32;
                    let avg: u8 = (sum as f32 / 3.0).ceil() as u8;
                    light.push(avg);
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        let mut jpg = Jpg {
            width,
            height,
            red,
            green,
            blue,
            light,
            binary: vec![],
            boundary: vec![],
        };
        jpg.binary = jpg.get_binary();
        jpg.boundary = jpg.get_boundary();
        jpg
    }
    pub fn export(&self, opt: &JpgOption) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        if opt.gray_scale {
            for y in 0..self.height {
                for x in 0..self.width {
                    let i = self.get_index_from_xy(x, y);
                    result.push(self.light[i]);
                    result.push(self.light[i]);
                    result.push(self.light[i]);
                }
            }
        } else {
            for y in 0..self.height {
                for x in 0..self.width {
                    let i = self.get_index_from_xy(x, y);
                    let red = (self.red[i] as f32 * opt.brightness * opt.red).ceil() as u8;
                    let green = (self.green[i] as f32 * opt.brightness * opt.green).ceil() as u8;
                    let blue = (self.blue[i] as f32 * opt.brightness * opt.blue).ceil() as u8;
                    result.push(red);
                    result.push(green);
                    result.push(blue);
                }
            }
        }
        result
    }
    pub fn export_bone(&self, opt: &JpgOption) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.binary[self.get_index_from_xy(x, y)] {
                    let red = (255.0 * opt.brightness * opt.red).ceil() as u8;
                    let green = (255.0 * opt.brightness * opt.green).ceil() as u8;
                    let blue = (255.0 * opt.brightness * opt.blue).ceil() as u8;
                    result.push(red);
                    result.push(green);
                    result.push(blue);
                } else {
                    for _ in 0..3 {
                        result.push(0);
                    }
                }
            }
        }
        result
    }
    pub fn export_boundary(&self, opt: &JpgOption) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.boundary[self.get_index_from_xy(x, y)] {
                    let red = (255.0 * opt.brightness * opt.red).ceil() as u8;
                    let green = (255.0 * opt.brightness * opt.green).ceil() as u8;
                    let blue = (255.0 * opt.brightness * opt.blue).ceil() as u8;
                    result.push(red);
                    result.push(green);
                    result.push(blue);
                } else {
                    for _ in 0..3 {
                        result.push(0);
                    }
                }
            }
        }
        result
    }
    pub fn remove_noise(&mut self, covered: u32, iteration: u32) {
        for _ in 0..iteration {
            let iter = std::iter::repeat(false).take(self.binary.len());
            let mut result: Vec<bool> = Vec::from_iter(iter);
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let filled_count = if self.binary[self.get_index_from_xy(x, y)] {
                        let mut count = 0;
                        for filled in [
                            self.binary[self.get_index_from_xy(x - 1, y)],
                            self.binary[self.get_index_from_xy(x + 1, y)],
                            self.binary[self.get_index_from_xy(x, y - 1)],
                            self.binary[self.get_index_from_xy(x, y + 1)],
                        ]
                        .iter()
                        {
                            if filled.to_owned() {
                                count += 1;
                            }
                        }
                        count
                    } else {
                        0
                    };

                    if filled_count >= covered {
                        result[self.get_index_from_xy(x, y)] = true;
                        if self.binary[self.get_index_from_xy(x - 1, y)] {
                            result[self.get_index_from_xy(x - 1, y)] = true;
                        }
                        if self.binary[self.get_index_from_xy(x + 1, y)] {
                            result[self.get_index_from_xy(x + 1, y)] = true;
                        }
                        if self.binary[self.get_index_from_xy(x, y - 1)] {
                            result[self.get_index_from_xy(x, y - 1)] = true;
                        }
                        if self.binary[self.get_index_from_xy(x, y + 1)] {
                            result[self.get_index_from_xy(x, y + 1)] = true;
                        }
                    }
                }
            }
            self.binary = result;
        }
    }
    pub fn get_xy_coords(&self, z: f32, use_boundary: bool) -> Vec<[f32; 3]> {
        let mut result: Vec<[f32; 3]> = vec![];
        let mut i = 0;
        let target = if use_boundary {
            &self.boundary
        } else {
            &self.binary
        };
        for filled in target.iter() {
            if *filled {
                let (x, y) = self.get_xy_from_index(i);
                result.push([x as f32, z, y as f32]);
            }
            i += 1;
        }
        result
    }

    // ====== private ====== //

    fn get_binary(&self) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];
        let critical_value = self.light.iter().max().unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.light[self.get_index_from_xy(x, y)] >= critical_value.clone() - DAMP {
                    result.push(true);
                } else {
                    result.push(false);
                }
            }
        }
        result
    }
    fn get_boundary(&self) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];
        let mut temp = self.binary[0];
        for y in 0..self.height {
            for x in 0..self.width {
                let filled = self.binary[self.get_index_from_xy(x, y)];
                if temp != filled {
                    temp = filled;
                    result.push(true);
                } else {
                    result.push(false);
                }
            }
        }

        temp = self.binary[0];
        for x in 0..self.width {
            for y in 0..self.height {
                let filled = self.binary[self.get_index_from_xy(x, y)];
                if temp != filled {
                    temp = filled;
                    result[self.get_index_from_xy(x, y)] = true;
                }
            }
        }

        result
    }
    fn get_index_from_xy(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
    fn get_xy_from_index(&self, index: u32) -> (u32, u32) {
        let x = index % self.width;
        let y = index / self.width;
        return (x, y);
    }
}
