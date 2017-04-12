extern crate rtlib;
extern crate image;

mod display;

use rtlib::vector::Vec3;
use rtlib::render::render;
use rtlib::camera::Film;
use rtlib::cameras::perspective::PerspectiveCamera;
use rtlib::scenes::pyramid::SCENE_PYRAMID;
use rtlib::textures::file::FileTexture;
use display::PngBuffer;

fn main() {
    let film :Film = Film {
        x_resolution: 480*3,
        y_resolution: 272*3,
        supersampling: 2,
        texture: &FileTexture{
            width: 403,
            height: 161,
            rgbdata: include_bytes!("../../textures/sky.rgb")
        },
        iso: 100,
    };

    let mut png = PngBuffer::new(&film);

    let mut cam = PerspectiveCamera::new(
        Vec3::new(-200.0,100.0,150.0),
        Vec3::new(0.0,0.0,20.0),
        film
    );

    cam.set_field_of_view(45);

    //let cam = OrthographicCamera::new(
        //Vec3::new(-154.0,0.0,0.0),
        //Vec3::zero(),
        //film
    //);

    render(&mut png, &cam, &SCENE_PYRAMID);

    png.write();
}
