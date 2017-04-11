use stm32f7::lcd::{Lcd as internalLcd, Color};
use rtlib::display::Display;
use rtlib::vector::Vec3;
use rtlib::math::min;

pub struct LcdDisplay {
    lcd: internalLcd
}

impl Display for LcdDisplay {
    fn set_pixel(&mut self, x :u16, y :u16, color: &Vec3) {
            self.lcd.print_point_color_at(x as u16,y as u16, LcdDisplay::color_to_internal(color));
    }

    fn reset(&mut self) {
        self.lcd.set_background_color(Color::rgb(0,0,0));
    }
}

impl LcdDisplay {
    pub fn init(lcd : internalLcd) -> LcdDisplay {
        LcdDisplay {
            lcd: lcd
        }
    }

    /// Converts a color in floating point vector
    /// representation to argb1555 format.
    fn color_to_internal(vec :&Vec3) -> u16 {
        Color::rgb(
            (min(1.0, vec.x) *255.0 + 0.5) as u8,
            (min(1.0, vec.y) *255.0 + 0.5) as u8,
            (min(1.0, vec.z) *255.0 + 0.5) as u8,
        ).to_argb1555()
    }
}

