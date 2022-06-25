use super::validatable::Validatable;

#[derive(Clone)]
pub struct Octave {
    pub type_name: String,
    pub persistance: f32,
    pub lacunarity: f32,
}

impl Validatable for Octave {
    fn adjust(&mut self) -> Octave {
        self.persistance = if self.persistance > 1f32 {
            1f32
        } else {
            self.persistance
        };
        self.clone()
    }
}

impl Octave {
    pub fn make_octave(type_name: String, persistance: f32, lacunarity: f32) -> Octave {
        Octave {
            type_name: type_name,
            persistance: persistance,
            lacunarity: lacunarity,
        }
        .adjust()
    }
}
