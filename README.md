# ShapeLogic Rust #

Image processing and coputer vision in Rust.

## Features ##

It is a command line application that can:

* Binary morphology
  * Dilate
  * Erode
* Blur image
* Convert to gray
* Edge detection
* Invert color
* Load image and save it in differnet image format
* Macro commands / combined commands
* Rotate and flip image
* Threshold with configurable limit

## Getting Started ##

Setup Rust then:

```
git clone https://github.com/sami-badawi/shapelogic-rust.git
cd shapelogic-rust
cargo build
./target/debug/shapelogic-rust --help
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e png -t edge
open img/output.png
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -t threshold -p 100
open img/output.bmp
```

## Image Operation Command List ##

Current list is:
```
blur, checkered, dilate, edge, erode, fliph, flipv, gray, invert, r270, r90, sobel_h, sobel_v and threshold
```

### Use Macros ###

To combine several commands use -m or --macro followed by semicolon separated commands.

```
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp --macro "edge;invert"

./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -m "edge;gray;invert;threshold 180"

./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -m "edge;gray;invert;threshold 180;erode"
```

The last macro command is what you would use to create a binary edge image that you could give to skeletonize algorithm.

## Status: v 0.3.0 Alpha ##

Currently ShapeLogic Rust is a command line application. 
With the simple macro commands ShapeLogic Rust can do useful work.

It has been remarkably easy to get started with Rust and image processing.

The [Image library](https://github.com/PistonDevelopers/image) is simple and works great. It makes it easy to get to the image buffer without 10 layers of indirection and heavy dependencies.

## Next Goals ##

* Start porting algorithms from ShapeLogic Scala
  * Skeletonize black and white image
  * Vectorize lines
* Figure out what to do about a GUI, possibly add web based GUI

## Sister Project ##

This project is meant to investigate how easy it is do image processing and computer vision programming in Rust, in comparison to using Scala and TypeScript/WebGL.

* [ShapeLogic Scala](https://github.com/sami-badawi/shapelogic-scala)
* [ShapeLogic TypeScript](https://github.com/sami-badawi/shapelogic-typescript)

All 3 are modern languages with very sophisticated type systems.

## GUI Ideas ##

To begin with the GUI can be rudementary.

### Simple Crossplatform ###

* Single page web application, possibly based on work in [ShapeLogic TypeScript](https://github.com/sami-badawi/shapelogic-typescript)
* Cursive / ncurses old but powerful GUI show image in browser

### Native Crossplatform ###

* Gtk og Gnome Rust bindings
* SDL2 game engine bindings
* Compile to WebAssembly and run in browser when compiler matures

These ideas are a little more involved.
