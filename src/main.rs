use csv;
mod generator;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const NOISE_SCALE: f64 = ((WIDTH + HEIGHT) as f64) * 0.1347;
const OCTAVES: usize = 2;
const PERSISTANCE: f64 = 0.25;
const LACUNARITY: f64 = 2f64;
const SEED: u32 = 2400;

fn main() {
    let noise_map = generator::generate_noise_map(generator::NoiseMapParams {
        width: WIDTH,
        height: HEIGHT,
        noise_scale: NOISE_SCALE,
        octaves: OCTAVES,
        persistance: PERSISTANCE,
        lacunarity: LACUNARITY,
        seed: SEED,
    });
    let mut writer =
        csv::Writer::from_path("gen/noise_map.csv").expect("Error while creating a writer");
    for row in noise_map.iter() {
        writer
            .write_record(row.iter().map(|num| num.to_string()))
            .expect("Error writing a row");
    }
    writer.flush().expect("Error flusing");
}
