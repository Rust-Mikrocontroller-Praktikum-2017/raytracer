extern crate rtlib;
extern crate image;

mod display;

use rtlib::vector::Vec3;
use rtlib::render::render;
use rtlib::camera::Film;
use rtlib::cameras::perspective::PerspectiveCamera;
use rtlib::cameras::orthographic::OrthographicCamera;
use rtlib::scenes::spheres::SCENE_SPHERE;
use rtlib::textures::color::NoTexture;
use rtlib::textures::file::FileTexture;
use rtlib::textures::noise::{LaticeNoise, EARTH_TEXTURE, NIGHT_SKY_TEXTURE};
use display::PngBuffer;

fn main() {
    let film :Film = Film {
        x_resolution: 480*3,
        y_resolution: 272*3,
        supersampling: 2,
        texture: &FileTexture{
            width: 1228,
            height: 613,
            rgbdata: include_bytes!("../../textures/latlong_palace.rgb")
        },
        iso: 100,
    };


    let mut png = PngBuffer::new(&film);

    
    let mut cam = PerspectiveCamera::new(
        Vec3::new(-400.0,-10.0,100.0),
        Vec3::zero(),
        film
    );

    cam.set_field_of_view(45);
    
    //let cam = OrthographicCamera::new(
        //Vec3::new(-154.0,0.0,0.0),
        //Vec3::zero(),
        //film
    //);

    render(&mut png, &cam, &SCENE_SPHERE);

    png.write();
}
