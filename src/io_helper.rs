extern crate image;

use std;
use std::fs::File;
use std::path::Path;

use self::image::GenericImage;
use self::image::ImageFormat;

// ================ extension_2_enum ================

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

// ================ input_to_output_name ================

/// General way to get from input name, given output name and extension to real output name
#[allow(dead_code)]
pub fn input_to_output_name(
    input_filename: &str,
    output_name: &str,
    extension: &str,
) -> std::string::String {
    let output_filename: &str = if output_name.is_empty() {
        input_filename
    } else {
        output_name
    };
    let output_filename_with_ending: String = if output_filename.ends_with(extension) {
        output_filename.to_owned()
    } else {
        format!("{}.{}", output_filename, extension)
    };
    output_filename_with_ending
}

#[test]
fn input_to_output_name_test() {
    let result_found = input_to_output_name("img/Lenna.jpg", "", "tiff");
    assert_eq!("img/Lenna.jpg.tiff".to_owned(), result_found)
}

// ================ image_format_converter ================

#[allow(dead_code)]
pub fn image_format_converter(input_filename: &str, output_name: &str, extension: &str) {
    let im = image::open(&Path::new(&input_filename)).unwrap();
    let output_filename = input_to_output_name(input_filename, output_name, extension);

    println!("Image: {} has dimensions {:?} and colors: {:?}", input_filename, im.dimensions(), im.color());

    let fout = &mut File::create(&Path::new(&output_filename)).unwrap();

    im.save(fout, image::PNG).unwrap();
}

// ================ jpg_2_png take out ================

/// Convert jpeg image to png image
#[allow(dead_code)]
pub fn jpg_2_png(input_filename: &str, output_name: &str) {
    let im = image::open(&Path::new(&input_filename)).unwrap();
    let output_filename = output_name.to_owned();

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

// ================ jpg_to_png take out ================

/// Convert jpeg image to png image reuse name
#[deprecated]
#[allow(dead_code)]
pub fn jpg_to_png(input_filename: &str) {
    let output_filename = format!("{}.png", input_filename);
    jpg_2_png(&input_filename, &output_filename)
}
