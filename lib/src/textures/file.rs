use texture::Texture;
use vector::Vec3;

pub struct FileTexture<'a> {
    pub width: u32,
    pub height: u32,
    pub rgbdata: &'a [u8]
}

impl<'a> Texture for FileTexture<'a> {
    fn get_texel(&self, u :f32, v:f32) -> Vec3 {
        let usubpixel = u * (self.width as f32);
        let vsubpixel = v * (self.height as f32);

        let x = usubpixel as u32;
        let y = vsubpixel as u32;

        let subx = usubpixel - x as f32;
        let suby = vsubpixel - y as f32;

        Vec3::lerp(
            &Vec3::lerp(&self.read_pos(x,y),   &self.read_pos(x+1,y),   subx),
            &Vec3::lerp(&self.read_pos(x,y+1), &self.read_pos(x+1,y+1), subx),
            suby
        )
    }
}

impl<'a> FileTexture<'a> {
    fn read_pos(&self, x: u32, y: u32) -> Vec3 {
        let index = ((y * self.width + x)*3) as usize;

        Vec3 {
            x: self.rgbdata[index]   as f32 / 255.0,
            y: self.rgbdata[index+1] as f32 / 255.0,
            z: self.rgbdata[index+2] as f32 / 255.0
        }
    }
}
