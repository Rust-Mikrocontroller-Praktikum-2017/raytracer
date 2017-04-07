use vector::Vec3;
use math::powf;
use camera::Camera;
use intersection::Intersection;
use scene::Scene;
use lcd::Color;

pub trait Material {
    /// True if this material is emitting light
    fn is_emitting(&self) -> bool;
    fn is_specular(&self) -> bool;
    fn is_transparent(&self) -> bool;
    fn evaluate_color(&self, cam :&Camera, isect :&Intersection, scene :&Scene) -> Vec3; 
}

pub fn vec3_to_argb1555(vec :&Vec3) -> u16 {
    Color::rgb(
        (vec.x*255.0 + 0.5) as u8,
        (vec.y*255.0 + 0.5) as u8,
        (vec.z*255.0 + 0.5) as u8,
    ).to_argb1555()
}

#[derive(Debug, Clone, Copy)]
pub struct ModifiedPhongModel {
    pub emission: Vec3,
    //k_emission_intensity: f32,
    //k_emission_color: &'a Vec3,
    //k_diffuse_specular: &'a Vec3,
    pub k_specular: Vec3,
    pub k_diffus: Vec3,
    pub k_ambient: Vec3,
    pub phong_exponent: f32,
    pub ior: f32,
    pub k_t: Vec3,
    pub transmitting: bool
}

impl Material for ModifiedPhongModel {
    fn is_emitting(&self) -> bool {
        self.emission.x > 0.0 || self.emission.y > 0.0 || self.emission.z > 0.0
    }

    fn is_specular(&self) -> bool {
        self.k_specular.x > 0.0 || self.k_specular.y > 0.0 || self.k_specular.z > 0.0
    }

    fn is_transparent(&self) -> bool {
        self.transmitting
    }

    fn evaluate_color(&self, cam :&Camera, isect :&Intersection, scene :&Scene) -> Vec3 {
        let mut intensity :Vec3 = self.emission.clone();

        for intersectable in scene.objects.iter() {
            let light = intersectable.get_material();

            if light.is_emitting() {
                let isect_pos = isect.get_position();
                let light_pos = intersectable.reduce_to_point();
                // TODO: premultiply this and cache
                //let light_emission = k_emission_color.mult(k_emission_color);
                intensity.inplace_add(&self.k_ambient.mult_vec(&light.emission));
                let mut l = light_pos.sub(&isect_pos);
                l.normalize();
                let n_dot_l = isect.normal.dot(&l);

                if n_dot_l > 0.0 {
                    intensity.inplace_add(self.k_diffus.mult_vec(&light.emission).inplace_mult(n_dot_l));

                    let mut r = isect.normal.mult(2.0 * n_dot_l);
                    r.inplace_sub(&l);
                    let mut v = cam.pos.sub(&isect_pos);
                    v.normalize();

                    let r_dot_v = r.dot(&v);

                    if r_dot_v > 0.0 {
                        intensity.inplace_add(&self.k_specular).inplace_mult_vec(&light.emission).inplace_mult(powf(r_dot_v, self.phong_exponent));
                    }
                }
            }
        }

        intensity
    }
}
