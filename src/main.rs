/// Start by just opening an image
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::GenericImage;

/// Function to test that image can open and save
fn jpg_to_png(filename: std::string::String) {
    let im = image::open(&Path::new(&filename)).unwrap();

    println!("dimensions {:?}", im.dimensions());

    println!("{:?}", im.color());

    let fout = &mut File::create(&Path::new(&format!("{}.png", filename))).unwrap();

    im.save(fout, image::PNG).unwrap();
}

fn main() {
    println!("Yes I know what you are thinking. More languages, more vaporware. :D");
    let filename = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        "./img/Lenna.jpg".to_owned()
    };
    jpg_to_png(filename);
}
