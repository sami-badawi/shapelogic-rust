/// image_create.rs
/// Helpers that create images

extern crate image;

use self::image::GrayImage;

/// Create empty gray image of given size
pub fn make_gray(imgx: u32, imgy: u32) -> GrayImage {
    let imgbuf = image::ImageBuffer::new(imgx, imgy);
    let output = image::ImageLuma8(imgbuf);
    output.to_luma()
}

// ---------------------------------------

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
