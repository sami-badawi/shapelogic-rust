extern crate image;

use std;
use std::fs::File;
use std::path::Path;

use self::image::GenericImage;
use self::image::ImageFormat;

use image_filter;
use image_operations;

// ================ extension_2_enum ================

/// Should maybe be made automatic
#[allow(dead_code)]
pub fn extension_2_enum(extension: &str) -> ImageFormat {
    let extension_string = extension.to_owned();
    let extension_lower = extension_string.to_lowercase();
    match extension_lower.as_str() {
        "bmp" => image::ImageFormat::BMP,
        "gif" => ImageFormat::GIF,
        "hdr" => image::ImageFormat::HDR,
        "ico" => image::ImageFormat::ICO,
        "jpg" | "jpeg" => ImageFormat::JPEG,
        "png" => ImageFormat::PNG,
        "ppm" => image::ImageFormat::PPM,
        "webp" => ImageFormat::WEBP,
        "tga" => image::ImageFormat::TGA,
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

pub fn image_transform(
    im: image::DynamicImage,
    transform: &str,
    parameter: &str,
) -> image::DynamicImage {
    let transform_lower = transform.trim().to_lowercase();
    let transform_ref: &str = &transform_lower;
    match transform_ref {
        "fliph" => im.fliph(),
        "flipv" => im.flipv(),
        "gray" => image::ImageLuma8(im.to_luma()),
        "r90" => im.rotate90(),
        "r270" => im.rotate270(),
        "blur" => image_filter::blur_operation(im),
        "checkered" => image_operations::checkered(im),
        "edge" => image_filter::edge_operation(im),
        "sobel_h" => image_filter::sobel_h_operation(im),
        "sobel_v" => image_filter::sobel_v_operation(im),
        "threshold" => {
            let limit: u8 = if parameter.is_empty() {
                128
            } else {
                parameter.parse::<u8>().unwrap()
            };
            image_operations::threshold(im, limit)
        }
        "invert" => {
            let mut rgb_img = im.clone();
            rgb_img.invert();
            rgb_img
        }
        "" => im,
        other => {
            println!("Unknown image tranformation: {}", other);
            im
        }
    }
}

// ================ image_format_converter ================

/// Change an image from one format to another
pub fn image_format_converter(
    input_filename: &str,
    output_name: &str,
    extension: &str,
    transform: &str,
    parameter: &str,
) {
    let im: image::DynamicImage = image::open(&Path::new(&input_filename)).unwrap();

    let output_filename = input_to_output_name(input_filename, output_name, extension);

    println!(
        "Image: {} has dimensions {:?} and colors: {:?}",
        input_filename,
        im.dimensions(),
        im.color()
    );
    let im_out = image_transform(im, transform, parameter);

    let fout = &mut File::create(&Path::new(&output_filename)).unwrap();
    let image_format = extension_2_enum(extension);
    println!("New image format: {:?}", image_format);

    im_out.save(fout, image_format).unwrap();
}
