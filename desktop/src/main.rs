extern crate rtlib;
extern crate image;

mod display;

use rtlib::vector::Vec3;
use rtlib::render::render;
use rtlib::camera::Film;
use rtlib::cameras::perspective::PerspectiveCamera;
//use rtlib::cameras::orthographic::OrthographicCamera;
use rtlib::scenes::spheres::SCENE_SPHERE;
use display::PngBuffer;

fn main() {
    let film :Film = Film {
        x_resolution: 480,
        y_resolution: 272,
        supersampling: 1,
        color: Vec3::new(0.0,0.4,0.8),
        iso: 100,
    };


    let mut png = PngBuffer::new(&film);

    
    let mut cam = PerspectiveCamera::new(
        Vec3::new(-200.0,0.0,0.5),
        Vec3::zero(),
        film
    );

    cam.set_focal_length(300.0);
    
    //let cam = OrthographicCamera::new(
        //Vec3::new(-154.0,0.0,80.0),
        //Vec3::zero(),
        //film
    //);

    render(&mut png, &cam, &SCENE_SPHERE);

    png.write();
}
