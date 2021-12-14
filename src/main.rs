use clap::{App, Arg};
use std::fs;
use std::path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("lutdinho")
        .version(std::env!("CARGO_PKG_VERSION"))
        .author(std::env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("FOLDER")
                .help("Folder where the .cube files are")
                .required(true)
                .min_values(1)
        )
        .get_matches();

    for path in matches.values_of("FOLDER").unwrap() {
        let directory = path::PathBuf::from(path);

        let mut images = vec![];
        let mut cube_files = vec![];
        for file in fs::read_dir(&directory)? {
            let file = file?;
            let full_name = file.path().to_str().unwrap().to_string();
            if !full_name.ends_with(".cube") {
                continue;
            }
            images.push(lutdinho::cube::Cube::parse(full_name)?);
            cube_files.push(file);
        }

        let lut_names = cube_files
            .iter()
            .map(|x| {
                x.file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace(".cube", "\\0")
            })
            .collect::<Vec<String>>()
            .join(" ");

        let directory_name = directory.file_name().unwrap().to_str().unwrap();
        let lut_name = directory_name.replace(" ", "_");
        let png_name = format!("{} MLUT.png", directory_name);
        let tilesize_xy = images[0].size;
        let tile_amount = images[0].n_images;
        let lut_amount = images.len();
        let fx_file = format!(
            include_str!("Template.fx"),
            lut_name, lut_names, png_name, tilesize_xy, tile_amount, lut_amount
        );
        println!("{}", fx_file);

        let generated_images: Vec<image::DynamicImage> = images
            .iter()
            .map(|x| x.generate_image(None).unwrap())
            .collect();
        let final_image = lutdinho::utils::vertical_stack(&generated_images)?;
        final_image.save(png_name)?;
        std::fs::write(format!("{} MLUT.fx", directory_name), fx_file)?;
    }

    Ok(())
}
