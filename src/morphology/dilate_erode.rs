/// Does dilate and errode of binary images
///
///
extern crate image;

#[allow(unused_imports)]
use self::image::{DynamicImage, GenericImage, Pixel};

#[allow(dead_code)]
pub fn dilate(image: &DynamicImage) -> DynamicImage {
    image.clone()
}

#[allow(dead_code)]
pub fn erode(image: &DynamicImage) -> DynamicImage {
    image.clone()
}
