use image::ImageBuffer;
use rayon::prelude::*;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Cube {
    pub size: usize,
    pub values: Vec<Color>,
    pub n_images: usize,
}

#[derive(Debug)]
pub struct Color(f32, f32, f32);

impl Cube {
    pub fn parse<P: AsRef<Path>>(filepath: P) -> Result<Cube, Box<dyn std::error::Error>> {
        let size_regex = Regex::new(r"LUT_3D_SIZE (\d*)")?;
        let value_regex = Regex::new(r"(\d(?:\.\d+(?:e-\d+)?)?) (\d(?:\.\d+(?:e-\d+)?)?) (\d(?:\.\d+(?:e-\d+)?)?)")?;
        let filepath = filepath.as_ref();
        let mut buf = String::new();
        let file_name = filepath.file_name().unwrap().to_string_lossy();
        let mut cube_file = BufReader::new(std::fs::File::open(filepath)?);
        let mut cube = Cube {
            size: 0,
            values: vec![],
            n_images: 0,
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
                "{}: Current values aren't divisible by size: {} {}",
                file_name,
                cube.values.len(),
                cube.size
            )
            .into());
        }

        cube.n_images = cube.values.len() / (cube.size * cube.size);

        Ok(cube)
    }

    pub fn generate_image(
        &self,
        target_size: Option<u32>,
    ) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        let mut images: Vec<image::DynamicImage> = vec![];

        for i in 0..self.n_images {
            let mut img = ImageBuffer::new(self.size as _, self.size as _);
            for y in 0..self.size {
                for x in 0..self.size {
                    let ix = (x + y * self.size) + self.size * self.size * i;
                    let pixel = image::Rgb::<u8>::from(&self.values[ix]);
                    img.put_pixel(x as _, y as _, pixel);
                }
            }
            images.push(image::DynamicImage::ImageRgb8(img));
        }

        if let Some(ts) = target_size {
            images
                .par_iter_mut()
                .for_each(|image| *image = image.resize(ts, ts, image::imageops::Triangle));
        }

        crate::utils::horizontal_stack(&images)
    }
}

impl From<&Color> for image::Rgb<u8> {
    fn from(color: &Color) -> Self {
        image::Rgb([
            (color.0 * 255.) as u8,
            (color.1 * 255.) as u8,
            (color.2 * 255.) as u8,
        ])
    }
}
