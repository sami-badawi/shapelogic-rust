/// Does dilate and errode of binary images
///
///
extern crate image;

#[allow(unused_imports)]
use self::image::{DynamicImage, GenericImage, GrayImage, Pixel};
use morphology::morphology_data::CYCLE_POINTS_2D;

#[allow(dead_code)]
const BACKGROUND_COLOR: u8 = 0;
#[allow(dead_code)]
const FOREGROUND_COLOR: u8 = 255;

// const cycle_Points

#[allow(dead_code)]
pub fn dilate(image: &GrayImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let imgbuf = image::ImageBuffer::new(width, height);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            for _xy in &CYCLE_POINTS_2D {
              let _p = image.get_pixel(x as u32, y as u32);

            }
        }
    }
    let res = image::ImageLuma8(imgbuf);
    res
}

#[allow(dead_code)]
pub fn erode(image: &DynamicImage) -> DynamicImage {
    image.clone()
}
