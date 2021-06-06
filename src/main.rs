use clap::{Arg, App};

fn main() {
    let matches = App::new("lutdinho")
        .version(std::env!("CARGO_PKG_VERSION"))
        .author(std::env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("FOLDER")
            .help("Folder where the .cube files are"))
        .get_matches();
}
