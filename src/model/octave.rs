pub struct Octave {
    x : f64,
    y : f64,
    type_name : String,
    frequency : u32,
    persistence : u32,
}

impl Octave{
    pub fn make_octave(x : f64, y : f64, type_name : String, frequency : u32, persistence : u32) -> Octave{
        Octave{
            x : x,
            y : y,
            type_name : type_name,
            frequency : frequency,
            persistence : persistence,
        }
    }
}
