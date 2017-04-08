#![allow(dead_code)]
#![feature(core_intrinsics)]

extern crate core;

fn main() {
    println!("If you see this, tests were not run!");
}

mod math;
mod vector;
mod intersection;
mod ray;
mod reflectionmodel;
mod camera;
mod scene;
