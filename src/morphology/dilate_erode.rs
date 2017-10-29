/// Does dilate and errode of binary images
///
///
extern crate image;

#[allow(unused_imports)]
use self::image::{DynamicImage, GenericImage, Pixel};

#[allow(dead_code)]
pub fn dilate(image: &DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let imgbuf = image::ImageBuffer::new(width, height);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let _p = image.get_pixel(x as u32, y as u32);
        }
    }
    let res = image::ImageLuma8(imgbuf);
    res
}

#[allow(dead_code)]
pub fn erode(image: &DynamicImage) -> DynamicImage {
    image.clone()
}
