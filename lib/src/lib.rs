#![allow(dead_code)]
#![feature(collections, alloc, core_intrinsics)]

#![cfg_attr(test, feature(test))]

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

#[cfg(test)]
extern crate test;
#[cfg(test)]
use test::Bencher;

#[bench]
fn no_supersampling(b: &mut Bencher) {
    supersample_bench(b, 1);
}

#[bench]
fn supersampling_4x(b: &mut Bencher) {
    supersample_bench(b, 2);
}

#[bench]
fn supersampling_9x(b: &mut Bencher) {
    supersample_bench(b, 3);
}

#[cfg(test)]
fn supersample_bench(b: &mut Bencher, multisample: u32) {
    use displays::none::None as NoDisplay;
    use camera::Film;
    use cameras::perspective::PerspectiveCamera;
    use render::render;
    use textures::color::NoTexture;

    use vector::Vec3;
    use scenes::spheres::SCENE_SPHERE;

    b.iter(|| {

        let film :Film = Film {
            x_resolution: 480,
            y_resolution: 272,
            supersampling: multisample,
            texture: &NoTexture { color: Vec3::new(0.0,0.1,0.2) },
            iso: 100,
        };


        let mut display = NoDisplay {};


        let mut cam = PerspectiveCamera::new(
            Vec3::new(-400.0,-10.0,500.0),
            Vec3::zero(),
            film
        );

        cam.set_field_of_view(45);

        render(&mut display, &cam, &SCENE_SPHERE);
    });
}
