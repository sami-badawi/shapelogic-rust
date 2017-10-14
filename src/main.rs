/// ShapeLogic Rust main
extern crate clap;

use clap::{App, Arg};
mod io_helper;

pub fn main() {
    let matches = App::new("ShapeLogic Rust")
        .version("0.1.0")
        .author("Sami Badawi")
        .about("ShapeLogic Rust, computer vision and image processing in Rust")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Input file name"),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("Output file name"),
        )
        .arg(
            Arg::with_name("extension")
                .short("e")
                .long("extension")
                .takes_value(true)
                .help("New image format extension"),
        )
        .get_matches();

    let filename = matches
        .value_of("file")
        .unwrap_or("./img/Lenna.jpg")
        .to_string();
    let output = matches.value_of("out").unwrap_or("").to_string();
    let extension = matches.value_of("extension").unwrap_or("png").to_string();

    println!("Run {}, out: {}, extension: {}", filename, output, extension);
    io_helper::image_format_converter(&filename, &output, &extension)
}
