/// Smaller image processing operations
extern crate image;

use self::image::{DynamicImage, GenericImage, Pixel};

pub fn threshold(img: DynamicImage, limit: u8) -> DynamicImage {
    let mut out_image = img.brighten(0);
    let height = img.height();
    let width = img.width();
    for x in 0..(width) {
        for y in 0..(height) {
            let px = img.get_pixel(x, y)
                .map(|v| if v <= limit { 0 } else { 255 });
            out_image.put_pixel(x, y, px);
        }
    }
    out_image
}

/// Turns an image into a checker pattern of normal and inverted
/// This is just for testing
pub fn checkered(img: DynamicImage) -> DynamicImage {
    let mut out_image = img.brighten(0);
    let height = img.height();
    let width = img.width();
    let mut x_bit_on: bool;
    let mut y_bit_on: bool;
    for x in 0..(width) {
        x_bit_on = x & 32 != 0;
        for y in 0..(height) {
            y_bit_on = y & 32 != 0;
            let px = img.get_pixel(x, y)
                .map(|v| if x_bit_on == y_bit_on { v } else { 255 - v });
            out_image.put_pixel(x, y, px);
        }
    }
    out_image
}
