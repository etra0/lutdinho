use image::ImageBuffer;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::Path;
use rayon::prelude::*;

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

    pub fn generate_image(&self, target_size: Option<u32>) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        let width = self.values.len() / self.size;
        let mut images: Vec<image::DynamicImage> = vec![];
        let n_images = self.values.len() / (self.size * self.size);

        for i in 0..n_images {
            let mut img = ImageBuffer::new(self.size as _, self.size as _);
            for y in 0..self.size {
                for x in 0..self.size {
                    let ix = (x + y*self.size) + self.size*self.size*i;
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

        let target_size = target_size.unwrap_or(self.size as _);
        let mut final_image = ImageBuffer::<image::Rgb<u8>, _>::new(target_size * (n_images as u32), target_size);
        for i in 0..n_images {
            for y in 0..target_size {
                for x in 0..target_size {
                    let cp = images[i].as_rgb8().unwrap().get_pixel(x, y);
                    final_image.put_pixel(x + target_size * (i as u32), y, cp.clone());
                }
            }
        }

        Ok(image::DynamicImage::ImageRgb8(final_image))
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
