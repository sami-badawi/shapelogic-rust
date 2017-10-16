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
