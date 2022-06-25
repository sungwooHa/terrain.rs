use model::{noise::Noise, octave::Octave, terrain::Terrain};

mod model;

pub fn test_runner1() -> Option<Vec<Vec<f64>>> {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    const SCALE: f64 = (WIDTH + HEIGHT) as f64 * 0.1347;

    let octaves: Vec<Octave> = vec![
        Octave::make_octave("terrain_1".to_string(), 0.25, 2f32),
        Octave::make_octave("terrain_2".to_string(), 0.5, 3f32),
    ];

    let noise = match Noise::make_noise(octaves, 2400) {
        Some(noise) => noise,
        None => {
            return None;
        }
    };

    let terrain = Terrain::make_terrain(noise, WIDTH, HEIGHT, SCALE);

    terrain.generate_noise_map()
}
