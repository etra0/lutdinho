use image;

pub fn horizontal_stack(images: &[image::DynamicImage]) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    if images.len() == 0 {
        return Err("images vec is empty".into());
    }

    let (width, height) = images[0].as_rgb8().ok_or("Something")?.dimensions();
    let mut final_image = image::RgbImage::new(width * (images.len() as u32), height);
    for (i, image) in images.iter().enumerate() {
        for y in 0..height {
            for x in 0..width {
                let cp = *image.as_rgb8().unwrap().get_pixel(x, y);
                final_image.put_pixel(x + width * (i as u32), y, cp);
            }
        }
    }

    return Ok(image::DynamicImage::ImageRgb8(final_image));
}

pub fn vertical_stack(images: &[image::DynamicImage]) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    if images.len() == 0 {
        return Err("images vec is empty".into());
    }
    let (width, height) = images[0].as_rgb8().ok_or("Something")?.dimensions();
    let mut final_image = image::RgbImage::new(width, height * (images.len() as u32));
    for (i, image) in images.iter().enumerate() {
        for y in 0..height {
            for x in 0..width {
                let cp = *image.as_rgb8().unwrap().get_pixel(x, y);
                final_image.put_pixel(x, y + height*(i as u32), cp);
            }
        }
    }

    return  Ok(image::DynamicImage::ImageRgb8(final_image));
}
