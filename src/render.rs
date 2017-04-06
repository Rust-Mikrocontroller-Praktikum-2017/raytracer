use camera::Camera;
use vector::Vec2;
use scene::Scene;
use intersection::Intersection;
use ray::Ray;

pub struct RenderBuffer {
    width  :i32,
    height :i32,
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
    let mut dir = cam.u * uv.u + cam.v * uv.v - cam.w * cam.d;
    dir.normalize();
    Ray::new(cam.pos, dir)
}

pub fn make_uv(buff :&RenderBuffer, cam :&Camera) -> Vec2 {
    Vec2 {
        u: ((cam.frame_left + (cam.frame_right - cam.frame_left)) as f64)  / (buff.width as f64),
        v: ((cam.frame_left + (cam.frame_right - cam.frame_left)) as f64)  / (buff.width as f64),
    }
}

pub fn render(buff :&RenderBuffer, cam :&Camera, scene :&Scene) {
    assert_eq!(cam.frame_left - cam.frame_right, buff.width);
    assert_eq!(cam.frame_top - cam.frame_bottom, buff.height);

    let uv = make_uv(buff, cam);

    for x in 0..(buff.width) {
        for y in 0..(buff.height) {
            let pixel_center = Vec2::new(x as f64 + 0.5, y as f64 + 0.5);
            let pixel_uv = uv * pixel_center;
            let primary_ray = gen_primary_ray(cam, &pixel_uv);

            let mut isect :Option<Intersection> = None;

            for intersectable in scene.objects.iter() {
                let tentative_isect = primary_ray.intersect(&intersectable);

                if let Some(curr_isect) = tentative_isect {
                    if curr_isect.t > 0.0 && (isect.is_none() || curr_isect.t < isect.unwrap().t) {
                        isect = tentative_isect;
                    }
                }
            }
        }
    }
}
