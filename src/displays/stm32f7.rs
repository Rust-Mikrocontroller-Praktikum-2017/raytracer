use lcd::{Lcd as internalLcd, Color};
use display::Display;
use vector::Vec3;

pub struct Lcd {
    lcd: internalLcd
}

impl Display for Lcd {
    fn set_pixel(&mut self, x :u16, y :u16, color: &Vec3) {
            self.lcd.print_point_color_at(x as u16,y as u16, Lcd::to_internal(color));
    }

    fn reset(&mut self) {
        self.lcd.set_background_color(Color::rgb(0,0,0));
    }
}

impl Lcd {
    pub fn init(lcd : internalLcd) -> Lcd {
        return Lcd {
            lcd: lcd
        }
    }

    /// Converts a color in floating point vector
    /// representation to argb1555 format.
    fn to_internal(vec :&Vec3) -> u16 {
        Color::rgb(
            (vec.x*255.0 + 0.5) as u8,
            (vec.y*255.0 + 0.5) as u8,
            (vec.z*255.0 + 0.5) as u8,
        ).to_argb1555()
    }
}

