/// Port of skeletonize function
///
/// This expect a very specific format that is a grayscala 1 byte image
extern crate image;

use self::image::{DynamicImage, GrayImage, ImageLuma8};
use morphology::image_create;

#[allow(dead_code)]
const BACKGROUND_COLOR: u8 = 0;
#[allow(dead_code)]
const FOREGROUND_COLOR: u8 = 255;

///
#[allow(dead_code)]
pub fn skeletonize(image: &GrayImage) -> Option<DynamicImage> {
    let (width, height) = image.dimensions();
    let mut buffer = image_create::make_raw_buffer(width, height);
    buffer[0] = FOREGROUND_COLOR;
    println!("buffer.len() {}", buffer.len());
    let imgbuf_opt = image_create::raw_buffer2image_buffer(width, height, buffer);
    match imgbuf_opt {
        Some(imgbuf) => Some(ImageLuma8(imgbuf)),
        None => None,
    }
}

// ------------------ Helper class not sure if needed ---------------------


#[allow(dead_code)]
pub struct Skeletonize<'a> {
    input_img: &'a GrayImage,
    inverted: bool,
    find_outline: bool,
    output_img: GrayImage,
}

impl<'a> Skeletonize<'a> {
    #[allow(dead_code)]
    pub fn new(input_image: &'a GrayImage) -> Skeletonize<'a> {
        let imgx = input_image.width();
        let imgy = input_image.height();
        let output_img = image_create::make_gray(imgx, imgy);

        Skeletonize {
            input_img: input_image,
            inverted: false,
            find_outline: true,
            output_img: output_img,
        }
    }
}
