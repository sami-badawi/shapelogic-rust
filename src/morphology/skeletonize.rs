/// Port of skeletonize function
///
/// This expect a very specific format that is a grayscala 1 byte image
extern crate image;

use self::image::GrayImage;

/// Just to test that the binary sub module is created correctly
#[allow(dead_code)]
pub fn hello_skeletonize() {
    println!("Hello {}", "Skeletonize")
}


#[allow(dead_code)]
pub struct Skeletonize {
    input_img: GrayImage,
    inverted: bool,
    find_outline: bool,
    output_img: GrayImage,
}

impl Skeletonize {
    #[allow(dead_code)]
    pub fn new(input_image: &GrayImage) -> Skeletonize {
        let imgx = input_image.width();
        let imgy = input_image.height();
        let imgbuf = image::ImageBuffer::new(imgx, imgy);
        let output = image::ImageLuma8(imgbuf);
        let output_img = output.to_luma();
        Skeletonize {
            input_img: input_image.clone(),
            inverted: false,
            find_outline: true,
            output_img: output_img,
        }
    }
}
