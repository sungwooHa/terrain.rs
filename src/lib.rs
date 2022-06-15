use model::octave::Octave;
use noise::{OpenSimplex, Seedable};

mod model;

use noise::{NoiseFn, OpenSimplex, Seedable};

pub fn generate_noise_map(map_info : model::map_generator::MapGenerator) -> Vec<Vec<f64>> {

    let perlin = OpenSimplex::new();
    perlin.set_seed(map_info.seed);

    let mut noise_map = map_info.make_noise_map();

    
    noise_map
}

pub fn apply_noise(mut noise_map : Vec<Vec<f64>>, octaves : Vec<Octave>) -> Vec<Vec<f64>> {
}