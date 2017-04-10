use rtlib::display::Display;
use rtlib::vector::Vec3;
use rtlib::camera::Film;
use rtlib::math::{min,max};

use std::vec::Vec;
use std::path::Path;
use image::{ImageBuffer, Rgb};

pub struct PngBuffer {
    buff: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl PngBuffer {
    pub fn new(film :&Film) -> PngBuffer {
        PngBuffer {
            buff: ImageBuffer::new(film.x_resolution, film.y_resolution)
        }
    }

    pub fn write(&self) {
        //let ref mut out = File::create().unwrap();
        let _ = self.buff.save(&Path::new("render.png")).unwrap();
    }
}

impl Display for PngBuffer {
    fn set_pixel(&mut self, x :u16, y :u16, color: &Vec3) {
        let pixel = Rgb { data: [
               (255.0 * max(0.0, min(1.0, color.x)) + 0.5) as u8,
               (255.0 * max(0.0, min(1.0, color.y)) + 0.5) as u8,
               (255.0 * max(0.0, min(1.0, color.z)) + 0.5) as u8
        ]};

        self.buff.put_pixel(x as u32,y as u32, pixel);
    }
    fn reset(&mut self) {
        unimplemented!();
    }
}
