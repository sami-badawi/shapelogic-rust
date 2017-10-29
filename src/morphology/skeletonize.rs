/// Port of skeletonize function
///
/// This expect a very specific format that is a grayscala 1 byte image
extern crate image;

use self::image::GrayImage;
use morphology::image_create;

/// Just to test that the binary sub module is created correctly
#[allow(dead_code)]
pub fn hello_skeletonize() {
    println!("Hello {}", "Skeletonize")
}
// ---------------------------------------


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
