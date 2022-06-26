#[derive(Clone)]
pub struct Noise {
    pub persistance: f32,
    pub lacunarity: f32,
    pub num_octaves: usize,
    pub seed: u64,
}

impl Noise {
    pub fn make_noise(persistance: f32, lacunarity: f32, num_octaves: usize, seed: u64) -> Noise {
        Noise {
            persistance,
            lacunarity,
            num_octaves,
            seed,
        }
    }
}
