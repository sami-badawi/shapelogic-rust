extern crate image;

use std;
use std::fs::File;
use std::path::Path;

use self::image::GenericImage;

/// Function to test that image can open and save
pub fn jpg_to_png(filename: std::string::String) {
    let im = image::open(&Path::new(&filename)).unwrap();

    println!("dimensions {:?}", im.dimensions());

    println!("{:?}", im.color());

    let fout = &mut File::create(&Path::new(&format!("{}.png", filename))).unwrap();

    im.save(fout, image::PNG).unwrap();
}
