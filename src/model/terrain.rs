use super::{noise_param::NoiseParam, pixel::Pixel};
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{prelude::StdRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct Terrain {
    noise_param: NoiseParam,
    pub width: usize,
    pub depth: usize,
    pub scale: f64,

    //about seed
    octave_offsets : Vec<(f64, f64)>, //octave 한개 한개의 control은 보류한다.
    perlin : OpenSimplex,


    //Result
    pub map : Vec<Vec<Pixel>>,
}

impl Terrain {
    pub fn make_terrain(noise_param: NoiseParam, width: usize, depth: usize, scale: f64) -> Terrain {
        Terrain {
            noise_param : noise_param,
            width :  width,
            depth : depth,
            scale : scale,
            perlin : OpenSimplex::new(),
            octave_offsets: Vec::new(),
            map : Vec::new(),
        }
    }


    fn generate_perlin(&mut self){
        self.perlin.set_seed(self.noise_param.seed as u32);
    }

    fn generate_octave(&mut self){
        let mut rng: StdRng = StdRng::seed_from_u64(self.noise_param.seed as u64);
        self.octave_offsets = (0..self.noise_param.num_octaves)
        .map(|_| {
            let x: f64 = rng.gen_range(-100000f64..100000f64);
            let y: f64 = rng.gen_range(-100000f64..100000f64);
            (x, y)
        })
        .collect();
    }


    fn generate_map(&mut self){
        let mut noise_map: Vec<Vec<Pixel>> = (0..self.width)
        .map(|_| (0..self.depth).map(|_| -> Pixel {Pixel::make_dummy()}).collect())
        .collect();

        self.generate_moisture_map();
        self.generate_height_map();
    }

    fn generate_moisture_map(&mut self) {

    }

    fn generate_height_map(&mut self) {

    }

    pub fn generate_noise_map(&self) {
        self.generate_perlin();
        self.generate_octave();
        self.generate_map();
    }

    pub fn generate_noise_map1(&self) -> Option<Vec<Vec<f64>>> {


        for y in 0..self.depth {
            for x in 0..self.width {
                let mut amplitude = 1f64;
                let mut frequency = 1f64;
                let mut noise_height = 0f64;

                for idx_octave in 0..self.noise_param.num_octaves {
                    let sample_x =
                        (x as f64) / self.scale * frequency + self.octave_offsets[idx_octave].0;
                    let sample_y =
                        (y as f64) / self.scale * frequency + self.octave_offsets[idx_octave].1;

                    let perlin_value = self.perlin.get([sample_x, sample_y]);
                    noise_height += perlin_value * amplitude;

                    amplitude *= self.noise_param.persistance as f64;
                    frequency *= self.noise_param.lacunarity as f64;
                }
                noise_map[x][y] = noise_height;
            }
        }

        Some(noise_map)
    }

    // }
}
