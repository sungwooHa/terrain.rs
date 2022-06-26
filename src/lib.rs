use model::attributes::{traits::*, *};
use model::{noise::Noise, terrain::Terrain};

mod model;

pub fn test_runner1() -> Option<Vec<Vec<f64>>> {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    const SCALE: f64 = (WIDTH + HEIGHT) as f64 * 0.1347;

    let noise = Noise::make_noise(0.25, 2f32, 3, 2400);

    let terrain = Terrain::make_terrain(noise, WIDTH, HEIGHT, SCALE);

    terrain.generate_noise_map()
}

pub fn generate_habitality_map() -> Option<Vec<Vec<Habitability>>> {
    let map = test_runner1();
    if let Some(map) = map {
        Some(
            map.iter()
                .map(|row| {
                    row.iter()
                        .map(|&col| Habitability::interpolate(col))
                        .collect()
                })
                .collect(),
        )
    } else {
        None
    }
}

pub fn generate_height_map() -> Option<Vec<Vec<Height>>> {
    let map = test_runner1();
    let height_interpolater = Height::interpolater(3f64, 24f64);

    if let Some(map) = map {
        Some(
            map.iter()
                .map(|row| row.iter().map(|&col| height_interpolater(col)).collect())
                .collect(),
        )
    } else {
        None
    }
}
