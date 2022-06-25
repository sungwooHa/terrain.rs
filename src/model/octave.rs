pub struct Octave {
    pub type_name: String,
    pub persistance: f32,
    pub lacunarity: f32,
}

impl Octave {
    pub fn make_octave(type_name: String, persistance: f32, lacunarity: f32) -> Octave {
        Octave {
            type_name: type_name,
            persistance: persistance,
            lacunarity: lacunarity,
        }
    }
}
