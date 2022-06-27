
#[derive(Clone)]
pub struct Pixel {
    height : f64,
    moisture : f64,
    habitance : bool,
}

impl Pixel {
    pub fn make_dummy() -> Pixel {
        Pixel {
             height: 0f64, 
             moisture: 0f64, 
             habitance: false, 
            }
    }
}