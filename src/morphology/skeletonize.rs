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
// ---------------------------------------


/// Create empty gray image of given size
/// Should be moved
pub fn make_gray(imgx: u32, imgy: u32) -> GrayImage {
    let imgbuf = image::ImageBuffer::new(imgx, imgy);
    let output = image::ImageLuma8(imgbuf);
    output.to_luma()
}

#[cfg(test)]
mod test {

    use super::{image, make_gray};

    #[test]
    fn make_gray_test() {
        let new_gray = make_gray(4, 4);
        let actual = new_gray.dimensions();
        assert_eq!((4, 4), actual);

        // let pixel1 = new_gray[(100, 100)];
        let pixel2 = new_gray.get_pixel(0, 0);
        assert_eq!(&image::Luma([0u8]), pixel2);
    }
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
        let output_img = make_gray(imgx, imgy);

        Skeletonize {
            input_img: input_image,
            inverted: false,
            find_outline: true,
            output_img: output_img,
        }
    }
}
