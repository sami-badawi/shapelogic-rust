extern crate image;

use std;
use std::fs::File;
use std::path::Path;

use self::image::GenericImage;

/// Convert jpeg image to png image
pub fn jpg_2_png(input_filename: std::string::String, output_filename: std::string::String) {
    let im = image::open(&Path::new(&input_filename)).unwrap();

    println!("dimensions {:?}", im.dimensions());

    println!("{:?}", im.color());

    let output_filename_with_ending = if output_filename.ends_with(".png") {
        output_filename
    } else {
        format!("{}.png", output_filename)
    };
    let fout = &mut File::create(&Path::new(&output_filename_with_ending)).unwrap();

    im.save(fout, image::PNG).unwrap();
}

/// Convert jpeg image to png image reuse name
pub fn jpg_to_png(input_filename: std::string::String) {
    let output_filename = format!("{}.png", input_filename);
    jpg_2_png(input_filename, output_filename)
}
