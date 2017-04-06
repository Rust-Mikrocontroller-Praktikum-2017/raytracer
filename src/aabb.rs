use vector::Vec3;

pub struct Aabb {
    pub min :Vec3,
    pub max :Vec3
}

impl Aabb {
    pub fn combine_with_aabb(&self, other :&Aabb) -> Aabb {
        Aabb {
            min: Vec3::min(&self.min, &other.min),
            max: Vec3::max(&self.max, &other.max),
        }
    }

    pub fn combine_with_point(&self, other :&Vec3) -> Aabb {
        Aabb {
            min: Vec3::min(&self.min, other),
            max: Vec3::max(&self.max, other),
        }
    }

    pub fn expand(&mut self, delta :f32) {
        let vec = Vec3::new(delta, delta, delta);
        self.min.inplace_sub(&vec);
        self.max.inplace_add(&vec);
    }
}
