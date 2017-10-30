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
#[allow(dead_code)]
pub fn make_gray(imgx: u32, imgy: u32) -> GrayImage {
    let imgbuf = make_gray_buffer(imgx, imgy);
    let output = image::ImageLuma8(imgbuf);
    output.to_luma()
}

#[allow(dead_code)]
pub fn make_raw_buffer(imgx: u32, imgy: u32) -> Vec<u8> {
  let final_length: usize = (imgx * imgy) as usize;
  let mut vec = Vec::with_capacity(final_length);
  vec.resize(final_length, 0u8);
  vec
}

#[allow(dead_code)]
pub fn raw_buffer2image_buffer(imgx: u32, imgy: u32, raw_buffer: Vec<u8>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
    ImageBuffer::from_vec(imgx,imgy,raw_buffer)
}

// ---------------------------------------

#[cfg(test)]
mod test {

    use self::image::{ImageBuffer, Luma};
    use super::{image, make_gray, make_gray_buffer};

    #[test]
    fn make_gray_buffer_test() {
        let new_gray = make_gray_buffer(4, 4);
        let actual = new_gray.dimensions();
        assert_eq!((4, 4), actual);
        let pixel2 = new_gray.get_pixel(0, 0);
        assert_eq!(&image::Luma([0u8]), pixel2);
    }

    #[test]
    fn make_gray_buffer_into_raw_test() {
        let new_gray = make_gray_buffer(4, 4);
        {
            let mut new_gray_data: Vec<u8> = new_gray.into_raw();
            assert_eq!(16, new_gray_data.len());
            new_gray_data[0] = 255u8;
            let new_gray_data2: Option<ImageBuffer<Luma<u8>, Vec<u8>>> = ImageBuffer::from_vec(4,4,new_gray_data);
            match new_gray_data2 {
                Some(data) => {
                    let pixel2 = data.get_pixel(0, 0);
                    assert_eq!(&image::Luma([255u8]), pixel2);
                }
                None => {
                    assert_eq!(1,2)
                }
            }
        }
        // This did not work, you cannot consume an image buffer with into_raw() and get use it after.
        //
        // let pixel2 = new_gray.get_pixel(0, 0);
        // assert_eq!(&image::Luma([255u8]), pixel2);
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
