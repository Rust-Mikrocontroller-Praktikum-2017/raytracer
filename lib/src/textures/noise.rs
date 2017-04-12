use texture::Texture;
use vector::Vec3;
use math::{max, min, modulo, rem};
use colormapping::{ColorMapping, EarthTones, SpaceAndStars};

pub struct Noise {}

impl Texture for Noise {
    fn get_texel(&self, u :f32, v: f32) -> Vec3 {
        // texture repeats every 800*800 pixel
        let noise = reproducable_randomness((u * 800.0) as u16, (v * 800.0) as u16, 1);
        Vec3 {
            x: noise, y: noise, z: noise
        }
    }
}

pub trait LatticeNoiseTexture : Texture {
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn sample(&self, u :u16, v :u16) -> f32;

    fn get_texel(&self, u :f32, v: f32) -> Vec3 {
        // repeat texture
        let ub = modulo(u, 1.0);
        let vb = modulo(v, 1.0);
        // clamp texture
        //let ub = max(0.0, min(u, 1.0));
        //let vb = max(0.0, min(v, 1.0));
        self.color_map(self.get_texel_channel(ub,vb))
    }

    fn get_texel_channel(&self, _u :f32, _v: f32) -> f32 {
        let u = _u * (self.get_width() as f32);
        let v = _v * (self.get_height() as f32);

        let u_int = u as u16;
        let v_int = v as u16;

        let u_decimal = u - (u_int as f32);
        let v_decimal = v - (v_int as f32);

        let sample_tl = self.sample(u_int, v_int);
        let sample_tr = self.sample(u_int + 1, v_int);
        let sample_bl = self.sample(u_int, v_int + 1);
        let sample_br = self.sample(u_int + 1, v_int + 1);

        let val_top = lerp(sample_tl, sample_tr, u_decimal);
        let val_bottom = lerp(sample_bl, sample_br, u_decimal);

        lerp(val_top, val_bottom, v_decimal)
    }

    fn color_map(&self, val :f32) -> Vec3 {
        //let v = max(0.0, min(1.0, val));
        //Vec3::new(v, v, v)
        Vec3::new(val,val,val)
    }
}

const RANDOM :[f32;97]= [
    0.3502347887,0.7707289142,0.8074737110,0.9560200738,0.8099103970,
    0.0465470354,0.7011022358,0.4005982395,0.9841398935,0.7324767543,
    0.8915973725,0.5589943791,0.5366404377,0.6833854732,0.5452557817,
    0.9633457016,0.3012300929,0.8753218745,0.1917415483,0.3045765056,
    0.2171197057,0.0276222202,0.3567521682,0.4055913557,0.8378382406,
    0.4501130535,0.1427198944,0.1277678544,0.8419720068,0.6169788697,
    0.9412445206,0.7957804674,0.2872914037,0.7556838805,0.0404204753,
    0.7483452659,0.0046914818,0.6939821440,0.2810013776,0.7052032899,
    0.3682301713,0.2739505057,0.1481559398,0.7643409518,0.2378122743,
    0.0305612787,0.3889278221,0.2849282554,0.5378392236,0.6260653762,
    0.5350779592,0.0231222080,0.4862766936,0.9304445341,0.9544514534,
    0.5065695571,0.5447665569,0.0122281342,0.2856594083,0.4065849255,
    0.5684405442,0.1764926921,0.9412099013,0.3988288599,0.4324758205,
    0.9949124388,0.6692077587,0.1703809503,0.8735547924,0.3008918430,
    0.4073889234,0.5233594139,0.0375347812,0.0862785566,0.4297229240,
    0.6331516114,0.7571938767,0.1913646704,0.8269879571,0.2900475789,
    0.9892561942,0.0246112806,0.2485182355,0.6948092398,0.8141068576,
    0.3344491680,0.8642747331,0.5247502835,0.9334799768,0.0000867655,
    0.2837620431,0.0422592508,0.3555656634,0.6729973376,0.5639365988,
    0.1498330680,0.3631261755,//0.3331190519,0.1122175372,0.7005139154,
    ];

fn reproducable_randomness(u :u16,v :u16, seed :u16) -> f32 {
    // mod using prime numbers to reduce visible
    // repetiton
    RANDOM[rem(seed as f32 * ((u as f32) + 277.0 * (v as f32)) + (seed as f32), 97.0) as usize]
}

pub struct LaticeNoise {
    pub width  :u16,
    pub height :u16,
    pub seed   :u16,
}

impl Texture for LaticeNoise {
    fn get_texel(&self, u :f32, v: f32) -> Vec3 {
        //let ub = max(0.0, min(u, 1.0));
        //let vb = max(0.0, min(v, 1.0));
        let ub = modulo(u, 1.0);
        let vb = modulo(v, 1.0);
        self.color_map(self.get_texel_channel(ub,vb))
    }
}

impl LatticeNoiseTexture for LaticeNoise {
    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_height(&self) -> u16 {
        self.height
    }

    fn sample(&self, u :u16,v :u16) -> f32 {
        reproducable_randomness(u,v, self.seed)
    }
}

pub struct Turbulence3<'a> {
    width: u16,
    height: u16,
    seed: u16,

    octave_1 : LaticeNoise,
    octave_2 : LaticeNoise,
    octave_1_weight :f32,
    octave_2_weight :f32,

    color_mapping: &'a ColorMapping
}

impl<'a> Texture for Turbulence3<'a> {
    fn get_texel(&self, u :f32, v: f32) -> Vec3 {
        //let ub = max(0.0, min(u, 1.0));
        //let vb = max(0.0, min(v, 1.0));
        let ub = modulo(u, 1.0);
        let vb = modulo(v, 1.0);
        self.color_mapping.color_map(self.get_texel_channel(ub,vb))
    }
}

impl<'a> LatticeNoiseTexture for Turbulence3<'a> {

    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_height(&self) -> u16 {
        self.height
    }

    fn sample(&self, u :u16,v :u16) -> f32 {
        reproducable_randomness(u,v, self.seed)
    }

    fn get_texel(&self, u :f32, v: f32) -> Vec3 {
        let ub = max(0.0, min(u, 1.0));
        let vb = max(0.0, min(v, 1.0));

        let val_octave_0 = self.get_texel_channel(ub,vb);
        let val_octave_1 = self.octave_1.get_texel_channel(ub,vb);
        let val_octave_2 = self.octave_2.get_texel_channel(ub,vb);

        let val = val_octave_0 + val_octave_1*self.octave_1_weight +
            val_octave_2*self.octave_2_weight;

        self.color_mapping.color_map(val)
    }
}

//fn u32_to_f32(a :u32) -> f32 {
//(a as f32) / (u32::max_value() as f32)
//}

fn lerp(a :f32, b :f32, p :f32) -> f32 {
    a * (1.0 - p) + b * p
}

//pub const MARMOR :Turbulence3 = Turbulence3 {

//};

pub const EARTH_TEXTURE :Turbulence3 = Turbulence3 {
    width:  20,
    height: 20,
    seed: 1,

    octave_1_weight: 0.50,
    octave_1 : LaticeNoise {
        width: 40,
        height: 40,
        seed: 422,
    },

    octave_2_weight: 0.25,
    octave_2 : LaticeNoise {
        width: 80,
        height: 80,
        seed: 1290,
    },

    color_mapping: &EarthTones {}
};

pub const NIGHT_SKY_TEXTURE :Turbulence3 = Turbulence3 {
    width:  100,
    height: 100,
    seed: 1,

    octave_1_weight: 1.0,
    octave_1 : LaticeNoise {
        width: 200,
        height: 200,
        seed: 999,
    },

    octave_2_weight: 1.0,
    octave_2 : LaticeNoise {
        width: 400,
        height: 400,
        seed: 9999,
    },

    color_mapping: &SpaceAndStars {}
};

pub const SAND_TEXTURE :LaticeNoise = LaticeNoise {
    width:  500,
    height: 500,
    seed: 100,
};
