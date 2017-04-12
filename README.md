__A simple ray tracer for embeded systems.__ Written in a week as an
introduction to the rust programming language. The ray tracer uses Turner
Whitted's formulation for recursion. Surfaces are represented using a modified
version of the phong reflection model. Textures are procedurally generated on
runtime.

[![Build Status](https://travis-ci.org/Rust-Mikrocontroller-Praktikum-2017/raytracer.svg?branch=master)](https://travis-ci.org/Rust-Mikrocontroller-Praktikum-2017/raytracer)


---
<p align=center><strong>Table of Contents</strong></p>

1. [Features and Showcase](#showcase)
2. [Building and Running on your OS](#building-and-running-on-your-os)
3. [Building for the Stm32f7 Microcontroller](#building-for-the-stm32f7-microcontroller)
4. [Executing Unit Tests](#executing-unit-tests)
5. [Benchmarking](#benchmarking)

---

### Features and Showcase

Swipe left, right, up or down to rotate the camera around the scene. Use the button
on the backside of the board to cycle through the available scenes.

![increasing reflectivity](./showcase/increasing_reflectivity.png)

_Shown above are spheres with increasing reflection coefficient._

![different texture mappings](./showcase/texture_mappings.png)

_Different methods to apply a tiled 2D texture to an object._

### Building and Running on your OS

Running the commands below, will build and run the ray tracer. The result will
be written to a file called `render.png` in your current working directory.

```
cd desktop && cargo run
```

### Building for the Stm32f7 Microcontroller

Run the commands below and [upload the program as described
here](https://github.com/embed-rs/stm32f7-discovery/blob/master/README.md).
Compiling in release mode reduces the time taken for rendering tasks
significantly.

```
cd stm32f7 && xargo build --release
```

### Executing Unit Tests

To execute unit tests run the following commands:

```
cd lib && cargo test
```

### Benchmarking

You can benchmark the application using the commands shown below. A simple
scene should take less than 10 milliseconds on a desktop system. Rendering
on the microcontroller should take about 10 seconds.

```
cd lib && cargo test
```
