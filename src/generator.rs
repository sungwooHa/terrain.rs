use noise::{NoiseFn, OpenSimplex, Seedable};

use rand::prelude::*;

pub struct NoiseMapParams {
    pub width: usize,
    pub height: usize,
    pub noise_scale: f64,
    pub octaves: usize,
    pub persistance: f64,
    pub lacunarity: f64,
    pub seed: u32,
}

pub fn generate_noise_map(props: NoiseMapParams) -> Vec<Vec<f64>> {
    let NoiseMapParams {
        width,
        height,
        noise_scale,
        octaves,
        persistance,
        lacunarity,
        seed,
    } = props;

    let perlin = OpenSimplex::new();
    perlin.set_seed(seed);

    let mut noise_map: Vec<Vec<f64>> = (0..width)
        .map(|_| (0..height).map(|_| 0f64).collect())
        .collect();

    let mut rng: StdRng = StdRng::seed_from_u64(seed as u64);
    let octave_offsets: Vec<(f64, f64)> = (0..octaves)
        .map(|_| {
            let x: f64 = rng.gen_range(-100000f64..100000f64);
            let y: f64 = rng.gen_range(-100000f64..100000f64);
            (x, y)
        })
        .collect();

    for y in 0..height {
        for x in 0..width {
            let mut amplitude = 1f64;
            let mut frequency = 1f64;
            let mut noise_height = 0f64;

            for octave in 0..octaves {
                let sx = (x as f64) / noise_scale * frequency + octave_offsets[octave].0;
                let sy = (y as f64) / noise_scale * frequency + octave_offsets[octave].1;

                let perlin_value = perlin.get([sx, sy]);
                noise_height += perlin_value * amplitude;
                amplitude *= persistance;
                frequency *= lacunarity;
            }
            noise_map[x][y] = noise_height;
        }
    }
    noise_map
}
