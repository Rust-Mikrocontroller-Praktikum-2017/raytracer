#![allow(dead_code)]
#![feature(core_intrinsics)]

extern crate core;

fn main() {
    println!("If you see this, tests were not run!");
}

mod vector;
mod math;
mod camera;
mod cameras;
mod scene;
mod scenes;
mod render;
mod intersection;
mod intersectables;
mod ray;
mod reflectionmodel;
mod display;
mod displays;
mod random;
mod texture;
mod textures;
