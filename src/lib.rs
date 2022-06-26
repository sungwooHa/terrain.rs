use model::{noise::Noise, octave::Octave, terrain::Terrain};

mod model;

pub fn test_runner1() -> Option<Vec<Vec<f64>>> {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    const SCALE: f64 = (WIDTH + HEIGHT) as f64 * 0.1347;

    let noise = Noise::make_noise(0.25, 2f32, 3, 2400);

    let terrain = Terrain::make_terrain(noise, WIDTH, HEIGHT, SCALE);

    terrain.generate_noise_map()
}
