/// image_create.rs
/// Helpers that create images
extern crate image;

use self::image::{GrayImage, ImageBuffer, Luma};

/// The ImageBuffer is the main part of an image
/// It somehow defaults to using Luma<u8> which is gray
pub fn make_gray_buffer(imgx: u32, imgy: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let imgbuf = image::ImageBuffer::new(imgx, imgy);
    imgbuf
}

/// Create empty gray image of given size
pub fn make_gray(imgx: u32, imgy: u32) -> GrayImage {
    let imgbuf = make_gray_buffer(imgx, imgy);
    let output = image::ImageLuma8(imgbuf);
    output.to_luma()
}

// ---------------------------------------

#[cfg(test)]
mod test {

    use super::{image, make_gray, make_gray_buffer};

    #[test]
    fn make_gray_buffer_test() {
        let new_gray = make_gray_buffer(4, 4);
        let actual = new_gray.dimensions();
        assert_eq!((4, 4), actual);

        // let pixel1 = new_gray[(100, 100)];
        let pixel2 = new_gray.get_pixel(0, 0);
        assert_eq!(&image::Luma([0u8]), pixel2);
    }

    #[test]
    fn make_gray_buffer_into_raw_test() {
        let new_gray = make_gray_buffer(4, 4);
        let new_gray_data: Vec<u8> = new_gray.into_raw();
        assert_eq!(16, new_gray_data.len());
    }

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
