extern crate image;

use std;
use std::fs::File;
use std::path::Path;

use self::image::GenericImage;
use self::image::ImageFormat;

/// Should be made automatic
#[allow(dead_code)]
pub fn extension_2_enum(extension: &str) -> ImageFormat {
    let extension_string = extension.to_owned();
    let extension_lower = extension_string.to_lowercase();
    match extension_lower.as_str() {
        "png" => ImageFormat::PNG,
        "jpg" | "jpeg" => ImageFormat::JPEG,
        "tiff" => ImageFormat::TIFF,
        _ => ImageFormat::PNG,
    }
}

#[test]
fn extension_2_enum_test_png() {
    let result_found = extension_2_enum("png");
    assert_eq!(ImageFormat::PNG, result_found)
}

#[test]
fn extension_2_enum_test_tiff() {
    let result_found = extension_2_enum("tiff");
    assert_eq!(ImageFormat::TIFF, result_found)
}

/// General way to get from input name, given output name and extension to real output name
#[allow(dead_code)]
pub fn input_to_output_name(
    input_filename: &str,
    output: &str,
    extension: &str,
) -> std::string::String {
    let output_filename: &str = if output.is_empty() {
        input_filename
    } else {
        output
    };
    let output_filename_with_ending: String = if output_filename.ends_with(extension) {
        output_filename.to_owned()
    } else {
        format!("{}.{}", output_filename, extension)
    };
    output_filename_with_ending
}

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
