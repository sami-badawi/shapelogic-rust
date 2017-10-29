/// Does dilate and errode of binary images
///
///
extern crate image;

#[allow(unused_imports)]
use self::image::{DynamicImage, GenericImage, GrayImage, ImageLuma8, Luma, Pixel};
use morphology::morphology_data::CYCLE_POINTS_2D;

#[allow(dead_code)]
const BACKGROUND_COLOR: u8 = 0;
#[allow(dead_code)]
const FOREGROUND_COLOR: u8 = 255;

// const cycle_Points

#[allow(dead_code)]
pub fn dilate(image: &GrayImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut gray_value: u8 = BACKGROUND_COLOR;
            for xy in &CYCLE_POINTS_2D {
                let p = image.get_pixel(x + xy.0 as u32, y + xy.1 as u32);
                if p.data == [FOREGROUND_COLOR] {
                    gray_value = FOREGROUND_COLOR;
                    break;
                }
            }
            imgbuf.put_pixel(x as u32, y as u32, Luma { data: [gray_value] });
        }
    }
    let res = image::ImageLuma8(imgbuf);
    res
}

#[allow(dead_code)]
pub fn dilate_dynamic(image: &DynamicImage) -> DynamicImage {
    match *image {
        ImageLuma8(ref gray_image) => dilate(gray_image),
        _ => {
            let color_type = image.color();
            println!(
                "color_type of image not right for dilate: {:?}",
                &color_type
            );
            image.clone()
        }
    }
}

#[allow(dead_code)]
pub fn erode(image: &DynamicImage) -> DynamicImage {
    image.clone()
}
