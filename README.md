# ShapeLogic Rust #

Image processing and coputer vision in Rust.

## Features ##

It is a command line application that can:

* Blur image
* Convert to gray
* Edge detection
* Invert color
* Load image and save it in differnet image format
* Rotate and flip image
* Threshold with configurable limit

## Getting Started ##

Setup Rust then:

```
git clone https://github.com/sami-badawi/shapelogic-rust.git
cd shapelogic-rust
cargo build
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e png -t edge
open img/output.png
./target/debug/shapelogic-rust --file img/Lenna.jpg --out img/output -e bmp -t threshold -p 100
open img/output.bmp
```

## Image Operation Command List ##

Current list is:
```
blur, checkered, edge, fliph, flipv, gray, invert, r270, r90, sobel_h, sobel_v and threshold
```

## Status: v 0.2.0 Pre-alpha ##

Currently ShapeLogic Rust is a command line application. 

It has been remarkably easy to get started with Rust and image processing.

The [Image library](https://github.com/PistonDevelopers/image) is simple and works great. It makes it easy to get to image buffer without 10 layers of indirection and heavy dependencies.

## Goals ##

* Get some image processing algorithms working from command line script
* Make a little macro language so you can run several commands in a row
* Start porting algorithms from ShapeLogic Scala
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
