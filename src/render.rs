use camera::Camera;
use vector::Vec3;
use scene::Scene;
use intersection::Intersection;
use ray::Ray;
use lcd::Lcd;
use reflectionmodel::{Material,vec3_to_argb1555};

pub fn render(lcd :&mut Lcd, cam :&Camera, scene :&Scene) {

    for y in 0..(cam.get_film().y_resolution) {
        for x in 0..(cam.get_film().x_resolution) {
            let primary_ray = cam.gen_primary_ray(x as f32 + 0.5, y as f32 + 0.5);

            let color = raytrace(&primary_ray, cam, scene);
            lcd.print_point_color_at(x as u16,y as u16,vec3_to_argb1555(&color));
        }
    }
}

fn raytrace(ray: &Ray, cam: &Camera, scene: &Scene) -> Vec3 {
    let mut color = Vec3::zero();

    let mut isect :Option<Intersection> = None;

    for intersectable in scene.objects.iter() {
        let tentative_isect = intersectable.intersect(&ray);

        if let Some(curr_isect) = tentative_isect {
            if curr_isect.t > 0.0 && (isect.is_none() || curr_isect.t < isect.unwrap().t) {
                isect = tentative_isect;
            }
        }
    }

    if let Some(actual_isect) = isect {
        let material = actual_isect.material;
        color = color.add(&material.evaluate_color(cam, &actual_isect, scene));

        let new_origin = actual_isect.get_position();
        if material.is_specular() {
            let new_dir = ray.direction.reflect(&actual_isect.normal);
            let new_ray = Ray::new(new_origin, new_dir);
            color = actual_isect.material.k_specular.mult_vec(&color.add(&raytrace(&new_ray,cam,scene)));
        }
        if material.transmitting {
            let refracted = ray.direction.refract(&actual_isect.normal, material.ior, false);
            match refracted {
                Some(new_dir) => {
                    let new_ray = Ray::new(new_origin,new_dir);
                    color = color.add(&actual_isect.material.k_t.mult_vec(&raytrace(&new_ray,cam,scene)))
                },
                None          => {
                    let new_dir = ray.direction.reflect(&actual_isect.normal);
                    let new_ray = Ray::new(new_origin,new_dir);
                    color = color.add(&actual_isect.material.k_specular.mult_vec(&raytrace(&new_ray,cam,scene)))
                }
            }
        }

        color
    } else {
        Vec3::new(0.0,0.0,1.0)
    }
}
