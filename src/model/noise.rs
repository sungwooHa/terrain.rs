use super::octave::Octave;

#[derive(Clone)]
pub struct Noise {
    pub octaves: Vec<Octave>,
    pub seed: u64,
}

impl Noise {
    pub fn make_noise(octaves: Vec<Octave>, seed: u64) -> Option<Noise> {
        if octaves.is_empty() {
            None
        } else {
            Some(Noise {
                octaves,
                seed,
            })
        }
    }

    //이름 중복을 검사해야할까?
    pub fn add_octave(&mut self, octave: Octave) {
        self.octaves.push(octave)
    }

    pub fn append_octave(&mut self, mut octaves: Vec<Octave>) {
        self.octaves.append(&mut octaves)
    }

    pub fn remove_octave(&mut self, type_name: String) -> bool {
        for octave_idx in 0..self.octaves.len() {
            if self.octaves[octave_idx].type_name == type_name {
                self.octaves.remove(octave_idx);
                return true;
            }
        }

        false
    }
}
