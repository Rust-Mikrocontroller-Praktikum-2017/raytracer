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

pub fn make_uv(buff :&RenderBuffer, cam :&Camera) -> Vec2 {
    Vec2 {
        u: ((cam.frame_left + (cam.frame_right - cam.frame_left)) as f64)  / (buff.width as f64),
        v: ((cam.frame_left + (cam.frame_right - cam.frame_left)) as f64)  / (buff.width as f64),
    }
}

pub fn render(lcd :&mut Lcd, buff :&RenderBuffer, cam :&Camera, scene :&Scene) {
    assert_eq!(cam.frame_left - cam.frame_right, buff.width);
    assert_eq!(cam.frame_top - cam.frame_bottom, buff.height);

    let uv = make_uv(buff, cam);

    for x in 0..(buff.width) {
        for y in 0..(buff.height) {
            let pixel_center = Vec2::new(x as f64 + 0.5, y as f64 + 0.5);
            let pixel_uv = uv.mult_vec(&pixel_center);
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

            if let Some(actual_isect) = isect {
                lcd.print_point_color_at(x as u16,y as u16, conv_color(&actual_isect.material));
            }
        }
    }
}

fn conv_color(cool :&Color) -> u16 {
    let mut hex :u16 = 1 << 15;
    hex = hex | (((cool.red   >> 3) as u16) << 10)
              | (((cool.green >> 3) as u16) << 5)
              | ((cool.blue >> 3) as u16);
    hex
}
