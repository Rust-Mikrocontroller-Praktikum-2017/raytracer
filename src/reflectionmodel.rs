use vector::Vec3;
use math::powf;
use camera::Camera;
use intersection::Intersection;
use scene::Scene;

pub trait Material {
    /// True if this material is emitting light
    fn is_emitting(&self) -> bool;
    fn evaluate_color(&self, cam :&Camera, isect :&Intersection, scene :&Scene); 
}

pub struct ModifiedPhongModel<'a> {
    emission: &'a Vec3,
    //k_emission_intensity: f32,
    //k_emission_color: &'a Vec3,
    //k_diffuse_specular: &'a Vec3,
    k_specular: &'a Vec3,
    k_diffus: &'a Vec3,
    k_ambient: &'a Vec3,
    //is_metal: bool,
    phong_exponent: f32
}

impl<'a> Material for ModifiedPhongModel<'a> {
    fn is_emitting(&self) -> bool {
        self.emission.x > 0.0 || self.emission.y > 0.0 || self.emission.z > 0.0
    }

    fn evaluate_color(&self, cam :&Camera, isect :&Intersection, scene :&Scene) {
        let intensity :Vec3 = self.emission.clone();

        for intersectable in scene.objects.iter() {
            let light = intersectable.get_material();

            if light.is_emitting() {
                // TODO: premultiply this and cache
                //let light_emission = k_emission_color.mult(k_emission_color);
                intensity.inplace_add(self.k_ambient.mul_vec(light.emission));
                let L = light.position.sub(isect.position);
                L.normalize();
                let N_dot_L = isect.normal.dot(L);

                if N_dot_L > 0.0 {
                    intensity.inplace_add(self.k_diffus.mul_vec(light.emission).inplace_mul(N_dot_L));

                    let R = isect.normal.mult(2.0 * N_dot_L).inplace_sub(L);
                    let V = cam.position - isect.position;
                    V.normalize();

                    let R_dot_V = R.dot(V);

                    if R_dot_V > 0 {
                        intensity.inplace_add(self.k_specular).inplace_mul(light.emission).inplace_mul(powf(R_dot_V, self.phong_exponent));
                    }
                }
            }
        }
    }
}
