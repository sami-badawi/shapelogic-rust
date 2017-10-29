/// Port of skeletonize function
///
/// This expect a very specific format that is a grayscala 1 byte image
extern crate image;

use self::image::{DynamicImage, GrayImage, ImageLuma8};
use morphology::image_create;
use morphology::morphology_data::{TABLE2, TABLE};

// ------------------ Constants ---------------------

#[allow(dead_code)]
const BACKGROUND_COLOR: u8 = 0;
#[allow(dead_code)]
const FOREGROUND_COLOR: u8 = 255;

const VERBOSE_LOGGING: bool = true;
const MAX_PASS: u32 = 100;

// ------------------ Skeletonize ---------------------

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
    buffer_even: Vec<u8>,
    buffer_odd: Vec<u8>,
    inverted: bool,
    find_outline: bool,
    stride: u32,
}

impl<'a> Skeletonize<'a> {
    #[allow(dead_code)]
    pub fn new(input_image: &'a GrayImage) -> Skeletonize<'a> {
        let width = input_image.width();
        let raw0 = input_image.clone().into_raw();
        let raw1 = input_image.clone().into_raw();

        Skeletonize {
            input_img: input_image,
            buffer_even: raw0,
            buffer_odd: raw1,
            inverted: false,
            find_outline: true,
            stride: width,
        }
    }

    #[allow(dead_code)]
    pub fn make_buffer(&self) -> Vec<u8> {
        let (width, height) = self.input_img.dimensions();
        let buffer = image_create::make_raw_buffer(width, height);
        buffer
    }

    #[allow(dead_code)]
    pub fn thin(&mut self, pass: u32, table: &[u32]) -> u32 {
        let row_offset = self.stride as usize;
        let even = pass % 2 == 0;
        let (in_buffer, output_pixels) = if even {
            (&mut self.buffer_even, &mut self.buffer_odd)
        } else {
            (&mut self.buffer_odd, &mut self.buffer_even)
        };

        let mut p1: u8;
        let mut p2: u8;
        let mut p3: u8;
        let mut p4: u8;
        let mut p5: u8;
        let mut p6: u8;
        let mut p7: u8;
        let mut p8: u8;
        let mut p9: u8;


        let mut v: u8;
        let mut index: u32;
        let mut code: u32;
        let mut offset: usize;

        let mut pixels_removed: u32 = 0;
        let bg_color = BACKGROUND_COLOR;


        let (width, height) = self.input_img.dimensions();
        for y in 1..(height - 2) {
            offset = (1 + y as usize * row_offset) as usize;
            for _x in 1..(width - 2) {
                p5 = in_buffer[offset];
                v = p5;
                if v != bg_color {
                    p1 = in_buffer[offset - row_offset - 1];
                    p2 = in_buffer[offset - row_offset];
                    p3 = in_buffer[offset - row_offset + 1];
                    p4 = in_buffer[offset - 1];
                    p6 = in_buffer[offset + 1];
                    p7 = in_buffer[offset + row_offset - 1];
                    p8 = in_buffer[offset + row_offset];
                    p9 = in_buffer[offset + row_offset + 1];
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
                    if (pass & 1) == 1 {
                        //odd pass
                        if code == 2 || code == 3 {
                            v = bg_color;
                            pixels_removed += 1;
                        }
                    } else {
                        //even pass
                        if code == 1 || code == 3 {
                            v = bg_color;
                            pixels_removed += 1;
                        }
                    }
                }
                output_pixels[offset] = v;
                offset += 1;
            }
        }
        pixels_removed
    }

    #[allow(dead_code)]
    pub fn skeletonize(&mut self) -> Option<DynamicImage> {

        let mut pass: u32 = 0;
        for _ in 0..MAX_PASS {
            let mut pixels_removed = self.thin(pass, &TABLE);
            pass += 1;
            pixels_removed += self.thin(pass, &TABLE);
            pass += 1;
            if VERBOSE_LOGGING {
                println!("table: pass: {}, pixels_removed: {}", pass, pixels_removed);
            }
            if pixels_removed <= 0 {
                break;
            }
        }
        for _ in 0..MAX_PASS {
            // use a second table to remove "stuck" outputPixels
            let mut pixels_removed = self.thin(pass, &TABLE2);
            pass += 1;
            pixels_removed += self.thin(pass, &TABLE2);
            pass += 1;
            if VERBOSE_LOGGING {
                println!("table: pass: {}, pixels_removed: {}", pass, pixels_removed);
            }
            if pixels_removed <= 0 {
                break;
            }
        }

        let (width, height) = self.input_img.dimensions();
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
