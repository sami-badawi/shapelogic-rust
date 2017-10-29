/// Does dilate and erode of binary images
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

// ------------------------ Dilate ------------------------

pub fn dilate(image: &GrayImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for y in 1..(height - 2) {
        for x in 1..(width - 2) {
            let mut gray_value: u8 = BACKGROUND_COLOR;
            for xy in &CYCLE_POINTS_2D {
                let x_c = x as i32 + xy.0 as i32;
                let y_c = y as i32 + xy.1 as i32;
                let p = image.get_pixel(x_c as u32, y_c as u32);
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

// ------------------------ Erode ------------------------

pub fn erode(image: &GrayImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for y in 1..(height - 2) {
        for x in 1..(width - 2) {
            let mut gray_value: u8 = FOREGROUND_COLOR;
            for xy in &CYCLE_POINTS_2D {
                let x_c = x as i32 + xy.0 as i32;
                let y_c = y as i32 + xy.1 as i32;
                let p = image.get_pixel(x_c as u32, y_c as u32);
                if p.data == [BACKGROUND_COLOR] {
                    gray_value = BACKGROUND_COLOR;
                    break;
                }
            }
            imgbuf.put_pixel(x as u32, y as u32, Luma { data: [gray_value] });
        }
    }
    let res = image::ImageLuma8(imgbuf);
    res
}

pub fn erode_dynamic(image: &DynamicImage) -> DynamicImage {
    match *image {
        ImageLuma8(ref gray_image) => erode(gray_image),
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
