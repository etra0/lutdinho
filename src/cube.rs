use image::ImageBuffer;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Cube {
    pub size: usize,
    pub values: Vec<Color>,
}

#[derive(Debug)]
pub struct Color(f32, f32, f32);

impl Cube {
    pub fn parse<P: AsRef<Path>>(filepath: P) -> Result<Cube, Box<dyn std::error::Error>> {
        let size_regex = Regex::new(r"LUT_3D_SIZE (\d*)")?;
        let value_regex = Regex::new(r"(\d\.\d+) (\d\.\d+) (\d\.\d+)")?;
        let mut buf = String::new();
        let mut cube_file = BufReader::new(std::fs::File::open(filepath)?);
        let mut cube = Cube {
            size: 0,
            values: vec![],
        };

        while let Ok(read) = cube_file.read_line(&mut buf) {
            if let Some(capture) = size_regex.captures(&buf) {
                cube.size = capture[1].parse()?;
                break;
            }

            if read == 0 {
                return Err("file doesn't contains LUT_3D_SIZE".into());
            }

            buf.clear();
        }

        while let Ok(read) = cube_file.read_line(&mut buf) {
            if let Some(cap) = value_regex.captures(&buf) {
                cube.values
                    .push(Color(cap[1].parse()?, cap[2].parse()?, cap[3].parse()?));
            }

            if read == 0 {
                break;
            }

            buf.clear();
        }

        if (cube.values.len() % cube.size) != 0_usize {
            return Err(format!(
                "Current values aren't divisible by size: {} {}",
                cube.values.len(),
                cube.size
            )
            .into());
        }

        Ok(cube)
    }

    pub fn generate_image(&self) -> image::DynamicImage {
        let width = self.values.len() / self.size;
        let mut img = ImageBuffer::new(width as u32, self.size as u32);

        for (i, pixel) in self.values.iter().enumerate() {
            let s = i / (self.size * self.size);
            let x = (i % self.size) + self.size * s;
            let y = (i / self.size) % self.size;
            img.put_pixel(x as _, y as _, image::Rgb::<u8>::from(pixel));
        }
        // let (o_width, o_height) = img.dimensions();
        let img = image::DynamicImage::ImageRgb8(img);
        // // TODO: uncomment this (? after discussing it with gordinho
        // let img = img.resize(o_width * 2, o_height * 2, image::imageops::Triangle);

        return img;
    }
}

impl From<&Color> for image::Rgb<u8> {
    fn from(color: &Color) -> Self {
        return image::Rgb([
            (color.0 * 255.) as u8,
            (color.1 * 255.) as u8,
            (color.2 * 255.) as u8,
        ]);
    }
}
