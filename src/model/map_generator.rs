
use super::{octave::Octave, terrain::Terrain};

pub struct MapGenerator{
    pub width : usize,
    pub height : usize,
    pub scale : f64,
    pub octaves : Vec<Octave>,
    pub seed : u32,
}

impl MapGenerator {
    pub fn generate_map(width : usize, height : usize, scale : f64, seed : u32) -> MapGenerator{
        MapGenerator{
            octaves : Vec::new(),
            width : width,
            height : height,
            scale : scale,
            seed : seed,
        }
    }

    pub fn make_noise_map(&self) -> Terrain {
        (0..self.width).map(|_| (0..self.height).map(|_| 0f64).collect()).collect()
    }

    pub fn add_octave(&mut self, octave: Octave) {
        self.octaves.push(octave)
    }

    pub fn append_octave(&mut self, mut octaves : Vec<Octave>) {
        self.octaves.append(&mut octaves)
    }
}