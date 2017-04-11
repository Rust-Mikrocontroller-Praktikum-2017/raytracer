use display::Display;
use vector::Vec3;

pub struct None {}

impl Display for None {
    fn set_pixel(&mut self, _x :u16, _y :u16, _color: &Vec3) {}
    fn reset(&mut self) {}
}
