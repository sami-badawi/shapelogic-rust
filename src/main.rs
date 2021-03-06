/// ShapeLogic Rust main
extern crate clap;

use clap::{App, Arg};

mod morphology;
mod io_helper;
mod image_filter;
mod image_macro;
mod image_operations;
mod model_collection;

pub fn main() {
    let matches = App::new("ShapeLogic Rust")
        .version("0.4.1")
        .author("Sami Badawi")
        .about("ShapeLogic Rust, computer vision and image processing in Rust. Example: 
shapelogic-rust --file img/Lenna.jpg --out img/output -e png -t edge
shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -m \"edge;invert\"
shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -m \"threshold 100;invert\"
")
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
        .arg(
            Arg::with_name("macro")
                .short("m")
                .long("macro")
                .takes_value(true)
                .help("Macro concatenate several transforms with a ; between: e.g.: \"threshold 100;invert\" "),
        )
        .arg(
            Arg::with_name("transform")
                .short("t")
                .long("transform")
                .takes_value(true)
                .help("What transform to do on image. Subcommands: \n:blur, checkered, edge, fliph, flipv, gray, invert, r270, r90, sobel_h, sobel_v, threshold"),
        )
        .arg(
            Arg::with_name("parameter")
                .short("p")
                .long("parameter")
                .takes_value(true)
                .help("The parameter to use e.g. for threshold"),
        )
        .get_matches();

    let filename = matches
        .value_of("file")
        .unwrap_or("./img/Lenna.jpg")
        .to_string();
    let output = matches.value_of("out").unwrap_or("").to_string();
    let extension = matches.value_of("extension").unwrap_or("png").to_string();
    let transform = matches.value_of("transform").unwrap_or("").to_string();
    let parameter = matches.value_of("parameter").unwrap_or("").to_string();
    let macro_input = matches.value_of("macro").unwrap_or("").to_string();

    println!(
        "Run {}, out: {}, extension: {}, transform: {}, parameter: {}, macro_input: {}",
        filename,
        output,
        extension,
        transform,
        parameter,
        macro_input
    );
    if macro_input.len() == 0 {
      io_helper::image_format_converter(&filename, &output, &extension, &transform, &parameter)
    }
    else {
      io_helper::image_macro_converter(&filename, &output, &extension, &macro_input)
    }
}
