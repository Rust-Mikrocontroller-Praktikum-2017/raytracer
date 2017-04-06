#![allow(dead_code)]
#![feature(collections, ops)]

#![no_std]
#![no_main]

#[macro_use]
extern crate collections;
extern crate alloc;

mod vector;
//mod camera;
//mod scene;
//mod render;
//mod intersection;
//mod ray;

use vector::Vec3;

fn main() {
    let mut vec = Vec3::new(-1.0,1.0,0.0);
    vec.normalize();
    let mut normal = Vec3::new(0.0,1.0,0.0);
    normal.normalize();

    let reflected = vec.reflect(&normal);
    println!("reflecting vector {:?} results in {:?}", vec, reflected);
}
