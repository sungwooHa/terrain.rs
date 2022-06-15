
pub struct Terrain {
    pub noise_map : Vec<Vec<f64>>, 
    height_map : Vec<Vec<f64>>,
}

impl Terrain {
    pub fn generate(width : usize, height : usize, ) -> Terrain {
        Terrain{
            noise_map : {(0..width)
            .map(|_| (0..height).map(|_| 0f64).collect())
            .collect();
            },

            height_map : {(0..width)
                .map(|_| (0..height).map(|_| 0f64).collect())
                .collect();
                },
        }
    }
}