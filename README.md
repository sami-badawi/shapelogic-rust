# ShapeLogic Rust #

Image processing and computer vision in Rust.

## Features ##

It is a command line application that can:

* Binary morphology
* Dilate
* Erode
* Blur image
* Convert to gray
* Edge detection
* Invert color
* Load image and save it in different image format
* Macro commands / combined commands
* Rotate and flip image
* Skeleton
* Threshold with configurable limit

## Getting Started ##

Setup Rust then:

```
git clone https://github.com/sami-badawi/shapelogic-rust.git
cd shapelogic-rust
cargo build
cargo test
./target/debug/shapelogic-rust --help
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e png -t edge
open img/output.png

./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -t threshold -p 100
open img/output.bmp

./target/debug/shapelogic-rust --file img/shape.png --out img/output -e png -t skeleton -p false
```

## Image Operation Command List ##

Current list is:
```
blur, checkered, dilate, edge, erode, fliph, flipv, gray, invert, r270, r90, skeleton, sobel_h, sobel_v and threshold
```

### Use Macros ###

To combine several commands use -m or --macro followed by semicolon separated commands.

```
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp --macro "edge;invert"

./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -m "edge;gray;invert;threshold 180"

./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -m "edge;gray;invert;threshold 180;erode"
```

### Commands with Parameters ###

* skeleton true
* threshold 180

Command skeleton takes true or false depending on dark or light background.
Command threshold takes cutoff value.

The last macro command is what you would use to create a binary edge image that you could give to skeletonize algorithm.

## Status: v 0.4.1 Alpha ##

Currently ShapeLogic Rust is a command line application. 
It has combined / macro commands. ShapeLogic Rust can do useful work.
There are a few unit tests.

It has been remarkably easy to get started with Rust and image processing.

The [Image library](https://github.com/PistonDevelopers/image) is simple, works great and is easy to extend.

## Next Goals ##

* Resizing operations
* Port vectorize lines code from ShapeLogic Scala
* Figure out what deep learning library to use
* Figure out what to do about a GUI, possibly add web based GUI

## Sister Project ##

This project is meant to investigate how easy it is do image processing and computer vision programming in Rust, in comparison to using Scala and TypeScript/WebGL.

* [ShapeLogic Scala](https://github.com/sami-badawi/shapelogic-scala)
* [ShapeLogic TypeScript](https://github.com/sami-badawi/shapelogic-typescript)

All 3 are modern languages with very sophisticated type systems.

## GUI Ideas ##

To begin with the GUI can be rudimentary.

### Simple Cross platform ###

* Single page web application, possibly based on work in [ShapeLogic TypeScript](https://github.com/sami-badawi/shapelogic-typescript)
* Cursive / ncurses old but powerful GUI show image in browser

### Native Cross platform ###

* Gtk or Gnome Rust bindings
* SDL2 game engine bindings
* Compile to WebAssembly and run in browser when compiler matures

These ideas are a little more involved.
