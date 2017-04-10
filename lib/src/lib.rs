#![allow(dead_code)]
#![feature(collections, alloc, core_intrinsics)]

#![no_std]

extern crate collections;
extern crate alloc;

pub mod vector;
pub mod math;
pub mod camera;
pub mod cameras;
pub mod scene;
pub mod scenes;
pub mod render;
pub mod intersection;
pub mod intersectables;
pub mod ray;
pub mod reflectionmodel;
pub mod display;
pub mod displays;
pub mod random;
pub mod texture;
pub mod textures;
pub mod colors;
pub mod colormapping;
pub mod texturemapping;
