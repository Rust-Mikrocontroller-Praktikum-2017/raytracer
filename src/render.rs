use camera::Camera;
use vector::Vec2;
use scene::Scene;
use intersection::Intersection;
use ray::Ray;
use lcd::{Lcd,Color};

pub struct RenderBuffer {
    pub width  :i32,
    pub height :i32,
}
/// Generate a ray from the eye/camera through
/// the center of a pixel in the image buffer.
///
/// Ray Equation r(t) is given by r(t) = cam.pos + t*s
/// where
///     - s is the return of this function
///     - r(0) is the camera position
///     - r(1) is the center of the pixel
pub fn gen_primary_ray(cam :&Camera, uv :&Vec2) -> Ray {
    let mut dir = (cam.u.mult(uv.u))
            .add(&(cam.v.mult(uv.v)))
            .sub(&(cam.w.mult(cam.focal_dist)));
    dir.normalize();
    Ray::new(cam.pos, dir)
}

pub fn make_uv(buff :&RenderBuffer, cam :&Camera, x: f32, y: f32) -> Vec2 {
    Vec2 {
        u: cam.frame_left as f32 + ((cam.frame_right - cam.frame_left) as f32 * (x+0.5)) / (buff.width as f32),
        v: cam.frame_top as f32 + ((cam.frame_bottom - cam.frame_top) as f32 * (y+0.5)) / (buff.height as f32),
    }
}

pub fn render(lcd :&mut Lcd, buff :&RenderBuffer, cam :&Camera, scene :&Scene) {
    assert_eq!(cam.frame_left - cam.frame_right, buff.width);
    assert_eq!(cam.frame_top - cam.frame_bottom, buff.height);

    for x in 0..(buff.width) {
        for y in 0..(buff.height) {
            let pixel_uv = make_uv(buff, cam, x as f32, y as f32);
            let primary_ray = gen_primary_ray(cam, &pixel_uv);

            let mut isect :Option<Intersection> = None;

            for intersectable in scene.objects.iter() {
                let tentative_isect = intersectable.intersect(&primary_ray);

                if let Some(curr_isect) = tentative_isect {
                    if curr_isect.t > 0.0 && (isect.is_none() || curr_isect.t < isect.unwrap().t) {
                        isect = tentative_isect;
                    }
                }
            }

            match isect {
                Some(actual_isect) => lcd.print_point_color_at(x as u16, y as u16, actual_isect.material.to_argb1555()),
                None               => lcd.print_point_color_at(x as u16, y as u16, Color::rgb(0,0,255).to_argb1555())
            }
        }
    }
}
