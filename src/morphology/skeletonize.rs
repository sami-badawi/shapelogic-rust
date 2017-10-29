/// Port of skeletonize function
///
/// This expect a very specific format that is a grayscala 1 byte image
extern crate image;

use self::image::{DynamicImage, GrayImage, ImageLuma8};
use morphology::image_create;
use morphology::morphology_data::{TABLE2, TABLE};

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
    input_buffer: Vec<u8>,
    inverted: bool,
    find_outline: bool,
    stride: u32,
    pass: u32,
}

impl<'a> Skeletonize<'a> {
    #[allow(dead_code)]
    pub fn new(input_image: &'a GrayImage) -> Skeletonize<'a> {
        let width = input_image.width();
        let raw = input_image.clone().into_raw();

        Skeletonize {
            input_img: input_image,
            input_buffer: raw,
            inverted: false,
            find_outline: true,
            stride: width,
            pass: 0,
        }
    }

    #[allow(dead_code)]
    pub fn make_buffer(&self) -> Vec<u8> {
        let (width, height) = self.input_img.dimensions();
        let buffer = image_create::make_raw_buffer(width, height);
        buffer
    }

    #[allow(dead_code)]
    pub fn thin(&self) -> (Vec<u8>, bool) {
        let rowOffset = self.stride as usize;
        let table = &TABLE;

        let mut p1: u8 = 0;
        let mut p2: u8 = 0;
        let mut p3: u8 = 0;
        let mut p4: u8 = 0;
        let mut p5: u8 = 0;
        let mut p6: u8 = 0;
        let mut p7: u8 = 0;
        let mut p8: u8 = 0;
        let mut p9: u8 = 0;


        let mut v: u8 = 0;
        let mut index: u32 = 0;
        let mut code: u32 = 0;
        let mut offset: usize = 0;

        let mut pixels_removed: u32 = 0;
        let mut count: u32 = 100;
        let in_buffer = &self.input_buffer;
        let bg_color = BACKGROUND_COLOR;

        let mut outputPixels = self.make_buffer();
        let (width, height) = self.input_img.dimensions();
        for y in 1..(height - 2) {
            for x in 1..(width - 2) {
                p5 = in_buffer[offset];
                v = p5;
                if v != bg_color {
                p1 = in_buffer[offset - rowOffset - 1];
                p2 = in_buffer[offset - rowOffset];
                p3 = in_buffer[offset - rowOffset + 1];
                p4 = in_buffer[offset - 1];
                p6 = in_buffer[offset + 1];
                p7 = in_buffer[offset + rowOffset - 1];
                p8 = in_buffer[offset + rowOffset];
                p9 = in_buffer[offset + rowOffset + 1];
                index = 0;
                if p1 != bg_color {
                    index |= 1u32
                }
                if p2 != bg_color {
                    index |= 2u32
                }
                if p3 != bg_color {
                    index |= 4u32
                }
                if p6 != bg_color {
                    index |= 8u32
                }
                if p9 != bg_color {
                    index |= 16u32
                }
                if p8 != bg_color {
                    index |= 32u32
                }
                if p7 != bg_color {
                    index |= 64u32
                }
                if p4 != bg_color {
                    index |= 128u32
                }
                code = table[index as usize];
                  if (self.pass & 1) == 1 { //odd pass
                    if code == 2 || code == 3 {
                      v = bg_color;
                      pixels_removed += 1;
                    }
                  } else { //even pass
                    if code == 1 || code == 3 {
                      v = bg_color;
                      pixels_removed += 1;
                    }
                  }
                }
                outputPixels[offset] = v;
                offset += 1;
            }
        }
        (outputPixels, false)
    }

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
}
