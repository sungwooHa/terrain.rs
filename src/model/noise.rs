
pub struct Noise{
    octaves : Vec<Octave>,
    seed : u32,
}

impl Noise {
    pub fn make_noise(octaves : Vec<Octave>, seed : u32) -> Optional<Noise> {

        if octaves.is_empty()
            return {};

        Noise {
            octaves : octaves,
            seed : seed
        }
    }

    pub fn generate_noise_map(&self, map_width : usize, map_height : usize, scale : f64, ) -> Vec<Vec<f64>> {

        let perlin = OpenSimplex::new();
        perlin.set_seed(seed);

        let mut noise_map = (0..width)
            .map(|_| (0..height).map(|_| 0f64).collect())
            .collect();        

        for y in 0..map_height {
            for x in 0..map_width {
                sampleX = x / scale;
                sampleY = y / scale;
            }
        }
    }

    fn adjust_scale(scale : &f64) -> f64{
        let scale = scale < 0 ? 0 : 

    }
}