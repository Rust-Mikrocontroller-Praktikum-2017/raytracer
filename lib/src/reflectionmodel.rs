use vector::Vec3;
use math::{powi, HUGE_EPS};
use camera::Camera;
use intersection::Intersection;
use scene::Scene;
use texture::TextureMapping;
use ray::Ray;
//use lcd::Color;

pub trait Material {
    /// True if this material is emitting light
    fn is_emitting(&self, surface_pos :&Vec3) -> bool;
    fn is_specular(&self, surface_pos :&Vec3) -> bool;
    fn is_transmitting(&self, surface_pos :&Vec3) -> bool;
    fn evaluate_color(&self, cam :&Camera, isect :&Intersection, scene :&Scene) -> Vec3;
}

pub struct ModifiedPhongModel<'a> {
    pub emission: &'a TextureMapping,
    pub k_specular: &'a TextureMapping,
    pub k_diffus: &'a TextureMapping,
    pub k_ambient: &'a TextureMapping,
    pub k_t: &'a TextureMapping,

    pub phong_exponent: f32,
    pub ior: f32,
}

impl<'a> Material for ModifiedPhongModel<'a> {
    fn is_emitting(&self, surface_pos :&Vec3) -> bool {
        let val = self.emission.map_texture(surface_pos);
        val.x > 0.0 || val.y > 0.0 || val.z > 0.0
    }

    fn is_specular(&self, surface_pos : &Vec3) -> bool {
        let val = self.k_specular.map_texture(surface_pos);
        val.x > 0.0 || val.y > 0.0 || val.z > 0.0
    }

    fn is_transmitting(&self, surface_pos : &Vec3) -> bool {
        let val = self.k_t.map_texture(surface_pos);
        val.x > 0.0 || val.y > 0.0 || val.z > 0.0
    }

    fn evaluate_color(&self, cam :&Camera, isect :&Intersection, scene :&Scene) -> Vec3 {
        let isect_pos = isect.get_position();
        let mut intensity = self.emission.map_texture(&isect_pos);
        let mut first_light = true;

        for intersectable in scene.objects.iter() {
            let light = intersectable.get_material();
            let emission = intersectable.reduce_emission();

            if light.is_emitting(&isect_pos) {
                let light_pos = intersectable.reduce_to_point();
                if first_light {
                    let val_ambient = self.k_ambient.map_texture(&isect_pos);
                    intensity.inplace_add(&val_ambient.mult_vec(&emission.div(emission.max_norm())));
                    first_light = false;
                }
                let mut l = light_pos.sub(&isect_pos);
                let dist = l.length();
                l.normalize();
                let n_dot_l = isect.normal.dot(&l);
                let mut shadow = false;
                let shadow_ray = Ray::new(isect_pos.add(&isect.normal.mult(HUGE_EPS)), l);
                for shadow_isectable in scene.objects.iter() {

                    // TODO: should be replaced with a shadow_isectable == intersectable
                    // currently avoids any shadows by light sources, but we only want
                    // to disallow selfintersection of the light src
                    if shadow_isectable.is_light()  {
                        continue;
                    }

                    let shadow_isect = shadow_isectable.intersect(&shadow_ray);
                    if let Some(si) = shadow_isect {
                        // the ray starts 
                        if si.t > 0.0 {
                            shadow = true;
                        }
                    }
                }

                if n_dot_l > 0.0 && !shadow {
                    let val_diffus = self.k_diffus.map_texture(&isect_pos);
                    let mut diff = val_diffus.mult_vec(&emission);
                    diff.inplace_mult(n_dot_l / (dist * dist));
                    intensity.inplace_add(&diff);

                    let mut r = isect.normal.mult(2.0 * n_dot_l);
                    r.inplace_sub(&l);
                    r.normalize();
                    let mut v = cam.get_position().sub(&isect_pos);
                    v.normalize();

                    let r_dot_v = r.dot(&v);

                    if r_dot_v > 0.0 {
                        let val_specular = self.k_specular.map_texture(&isect_pos);
                        intensity.inplace_add(val_specular.mult_vec(&emission).inplace_mult(powi(r_dot_v, self.phong_exponent as u32) / (dist*dist)));
                    }
                }
            }
        }

        intensity
    }
}
