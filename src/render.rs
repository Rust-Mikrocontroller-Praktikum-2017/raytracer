use camera::Camera;
use vector::Vec3;
use scene::Scene;
use intersection::Intersection;
use ray::Ray;
use reflectionmodel::Material;
use math::EPS;
use display::Display;

pub fn render(display :&mut Display, cam :&Camera, scene :&Scene) {

    let film = cam.get_film();

    // note: we are supersampling a grid positioned at the top
    // left corner of the pixel, e.g. for the 2x2 supersampling:
    //
    // 0----X----X
    // |         |
    // 0    X    X
    // |         |
    // o----o----o
    //
    // where `X` are samples computed for the current pixel, and
    // `o` are samples computed by adjacent pixels.
    
    let supersampling = film.supersampling;
    let sample_dist = 1.0 / (supersampling as f32);

    for y in 0..(film.y_resolution) {
        for x in 0..(film.x_resolution) {
            let mut x_sample = x as f32;
            let mut y_sample = y as f32;
            let mut pixel_color = Vec3::zero();

            for _y_supixel in 0..supersampling {
                for _x_supixel in 0..supersampling {
                    x_sample += sample_dist;
                    y_sample += sample_dist;

                    let primary_ray = cam.gen_primary_ray(x_sample, y_sample);

                    let sample_color = raytrace(&primary_ray, cam, scene, 0);
                    pixel_color.inplace_add(&sample_color);
                }
            }

            pixel_color.inplace_div((supersampling * supersampling) as f32);

            let developed_color = film.develop(pixel_color, x as f32, y as f32);
            display.set_pixel(x as u16,y as u16, &developed_color);
        }
    }
}

fn raytrace(ray: &Ray, cam: &Camera, scene: &Scene, depth: u8) -> Vec3 {
    if depth > 5 {
        return cam.get_film().color;
    }
    let mut isect :Option<Intersection> = None;

    for intersectable in scene.objects.iter() {
        let tentative_isect = intersectable.intersect(ray);

        if let Some(curr_isect) = tentative_isect {
            if curr_isect.t > 0.0 && (isect.is_none() || curr_isect.t < isect.unwrap().t) {
                isect = tentative_isect;
            }
        }
    }

    if let Some(actual_isect) = isect {
        let mut color = Vec3::zero();
        let material = actual_isect.material;
        color.inplace_add(&material.evaluate_color(cam, &actual_isect, scene));

        let new_origin = actual_isect.get_position();

        // TODO: refactor to perform the texture lookup only once
        if material.is_specular(&new_origin) {
            let mut new_dir = ray.direction.reflect(&actual_isect.normal);
            new_dir.normalize();
            let new_ray = Ray::new(new_origin.add(&new_dir.mult(EPS)), new_dir);
            let spec_val = &material.k_specular.map_texture(&new_origin);
            color.inplace_add(&spec_val.mult_vec(&raytrace(&new_ray,cam,scene,depth+1)));
        }
        if material.is_transmitting(&new_origin) {
            let refracted = ray.direction.refract(&actual_isect.normal, material.ior, false);
            let t_val = material.k_t.map_texture(&new_origin);

            let new_ray : Ray;

            match refracted {
                Some(mut new_dir) => {
                    new_dir.normalize();
                    new_ray = Ray::new(new_origin.add(&new_dir.mult(EPS)),new_dir);
                },
                None          => {
                    let mut new_dir = ray.direction.reflect(&actual_isect.normal);
                    new_dir.normalize();
                    new_ray = Ray::new(new_origin.add(&new_dir.mult(EPS)),new_dir);
                }
            }

            color.inplace_add(&t_val.mult_vec(&raytrace(&new_ray,cam,scene,depth+1)));
        }

        color
    } else {
        cam.get_film().color
    }
}
