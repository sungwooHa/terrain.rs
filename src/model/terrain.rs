use super::{
    noise::Noise,
    validatable::{self, Validatable},
};
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{prelude::StdRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct Terrain {
    noise: Noise,
    pub width: usize,
    pub height: usize,
    pub scale: f64,
}

impl Validatable for Terrain {
    fn adjust(&mut self) -> Terrain {
        self.width = if self.width < 1 { 1 } else { self.width };
        self.height = if self.height < 1 { 1 } else { self.height };

        self.clone()
    }
}

impl Terrain {
    pub fn make_terrain(noise: Noise, width: usize, height: usize, scale: f64) -> Terrain {
        Terrain {
            noise,
            width,
            height,
            scale,
        }
        .adjust()
    }

    pub fn generate_noise_map(&self) -> Option<Vec<Vec<f64>>> {
        let perlin = OpenSimplex::new();
        perlin.set_seed(self.noise.seed as u32);

        let mut rng: StdRng = StdRng::seed_from_u64(self.noise.seed as u64);
        let octave_offsets: Vec<(f64, f64)> = (0..self.noise.octaves.len())
            .map(|_| {
                let x: f64 = rng.gen_range(-100000f64..100000f64);
                let y: f64 = rng.gen_range(-100000f64..100000f64);
                (x, y)
            })
            .collect();

        let mut noise_map: Vec<Vec<f64>> = (0..self.width)
            .map(|_| (0..self.height).map(|_| 0f64).collect())
            .collect();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut amplitude = 1f64;
                let mut frequency = 1f64;
                let mut noise_height = 0f64;

                for idx_octave in 0..self.noise.octaves.len() {
                    let sample_x =
                        (x as f64) / self.scale * frequency + octave_offsets[idx_octave].0;
                    let sample_y =
                        (y as f64) / self.scale * frequency + octave_offsets[idx_octave].1;

                    let perlin_value = perlin.get([sample_x, sample_y]);
                    noise_height += perlin_value * amplitude;

                    amplitude *= self.noise.octaves[idx_octave].persistance as f64;
                    frequency *= self.noise.octaves[idx_octave].lacunarity as f64;
                }
                noise_map[x][y] = noise_height;
            }
        }

        Some(noise_map)
    }

    // }
}
