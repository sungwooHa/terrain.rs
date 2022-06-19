use super::noise::Noise;
use noise::{NoiseFn, OpenSimplex, Seedable};
pub struct Terrain {
    noise: Noise,
    pub width: usize,
    pub height: usize,
    pub scale: f64,
}

impl Terrain {
    pub fn make_terrain(noise: Noise, width: usize, height: usize, scale: f64) -> Terrain {
        Terrain {
            noise: noise,
            width: width,
            height: height,
            scale: scale,
        }
    }

    pub fn generate_noise_map(&self) -> Vec<Vec<f64>> {
        let perlin = OpenSimplex::new();
        perlin.set_seed(self.noise.seed);

        let mut noise_map: Vec<Vec<f64>> = (0..self.width)
            .map(|_| (0..self.height).map(|_| 0f64).collect())
            .collect();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut amplitude = 1f64;
                let mut frequency = 1f64;
                let mut noise_height = 0f64;

                for octave in &self.noise.octaves {
                    let sample_x = (x as f64) / self.scale * frequency + octave.x;
                    let sample_y = (y as f64) / self.scale * frequency + octave.y;

                    let perlin_value = perlin.get([sample_x, sample_y]);
                    noise_height += perlin_value * amplitude;

                    amplitude *= octave.persistance as f64;
                    frequency *= octave.lacunarity as f64;
                }
                noise_map[x][y] = noise_height;
            }
        }
        noise_map
    }

    // }
}
