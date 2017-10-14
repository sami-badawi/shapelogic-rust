/// ShapeLogic Rust main
extern crate clap;

use clap::{App, Arg};
mod io_helper;

pub fn main() {
    let matches = App::new("ShapeLogic Rust")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
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
            Arg::with_name("extention")
                .short("e")
                .long("extention")
                .takes_value(true)
                .help("new image format extention"),
        )
        .get_matches();


    let filename = matches
        .value_of("file")
        .unwrap_or("./img/Lenna.jpg")
        .to_string();
    let output = matches.value_of("out").unwrap_or("").to_string();
    let extention = matches.value_of("extention").unwrap_or("png").to_string();

    println!("Input file: {}; Output fiel: {}", filename, output);

    println!("Yes I know what you are thinking. More languages, more vaporware. :D");
    println!("{}", filename);
    io_helper::image_format_converter(&filename, &output, &extention)
    // if output.is_empty() {
    //     io_helper::jpg_to_png(&filename);
    // } else {
    //     io_helper::jpg_2_png(&filename, &output);
    // }
}
