/// ShapeLogic Rust main
use std::env;
mod io_helper;

pub fn main() {
    println!("Yes I know what you are thinking. More languages, more vaporware. :D");
    let filename = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        "./img/Lenna.jpg".to_owned()
    };
    println!("{}", filename);
    io_helper::jpg_to_png(filename);
}
