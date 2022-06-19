pub struct Octave {
    pub x: f64,
    pub y: f64,
    pub type_name: String,
    pub persistance: u32,
    pub lacunarity: u32,
}

impl Octave {
    pub fn make_octave(
        x: f64,
        y: f64,
        type_name: String,
        persistance: u32,
        lacunarity: u32,
    ) -> Octave {
        Octave {
            x: x,
            y: y,
            type_name: type_name,
            persistance: persistance,
            lacunarity: lacunarity,
        }
    }
}
