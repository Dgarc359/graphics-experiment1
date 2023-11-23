// my local util
use rand::Rng;
use std::collections::HashSet;

pub fn linear2srgb(x: f32) -> u8 {
    let x = if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    };
    (x * 255.0 + 0.5).floor().clamp(0.0, 255.0) as u8
}

fn generate_random_u8(min: u8, max: u8) -> u8 {
    rand::thread_rng().gen::<u8>().clamp(min, max)
}

pub fn generate_random_unique_u8s(num_to_gen: u8, min: u8, max: u8) -> HashSet<u8> {
    let mut results: HashSet<u8> = HashSet::new();

    let mut i = 0;
    loop {
        if i >= num_to_gen {
            break;
        }
        let gen = generate_random_u8(min, max);
        if results.contains(&gen) {
            continue;
        } else {
            results.insert(gen);
            i = i + 1;
        }
    }
    results
}
