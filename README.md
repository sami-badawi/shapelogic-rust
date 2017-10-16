# ShapeLogic Rust

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

## Goals ##

* Get some image processing algorithms working from command line script
* Start porting algorithms from ShapeLogic Scala
* Figure out what to do about a GUI, possibly web based on

## Sister project ##

This project is meant investigate how easy it is do image processing and computer vision programming in Rust, in comparison to using Scala and TypeScript/WebGL.

* [ShapeLogic Scala](https://github.com/sami-badawi/shapelogic-scala)
* [ShapeLogic TypeScript](https://github.com/sami-badawi/shapelogic-typescript)

All 3 are modern languages with very sophisticated type systems.


## GUI Ideas ##

To begin with the GUI can be rudementary.

* Single page web application, possibly based on work in [ShapeLogic TypeScript](https://github.com/sami-badawi/shapelogic-typescript)
* Gtk og Gnome Rust bindings
* SDL2 game engine bindings
* Cursive / ncurses old but powerful GUI show image in browser
* Compile to WebAssembly and run in browser when compiler matures

## Status: v 0.2.0 Pre-alpha ##

Currently ShapeLogic Rust is a command line application. 

It has been remarkably easy to get started with Rust and Image lib works great.
