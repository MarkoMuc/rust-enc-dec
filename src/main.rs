use std::{env, fs, process::exit};

use rand::{rngs::StdRng, Rng, SeedableRng};

fn decode(path: &str) -> Vec<u8> {
    let mut contents = fs::read(path).expect("Failed to read.");
    let mut new_data: Vec<u8> = vec![0; contents.len()];
    let seed: u8;

    if !contents.contains(&0x4u8) {
        println!("Not encoded.");
        return contents;
    }

    seed = contents.pop().unwrap();
    contents.pop();

    let mut r = StdRng::seed_from_u64(seed as u64);
    let mut new_max: u64 = 2 * pow2u8(r.gen::<u8>() % 3) as u64;
    let mut start_index = 0;
    let mut counter = 0;

    for char in contents {
        if counter == new_max {
            start_index += counter;
            counter = 0;
            new_max = 2 * pow2u8(r.gen::<u8>() % 3) as u64;
        }

        if counter < new_max / 2 {
            new_data[(start_index + counter + new_max / 2) as usize] = char;
        } else if counter >= new_max / 2 {
            new_data[(start_index + counter - new_max / 2) as usize] = char;
        }

        counter += 1;
    }

    new_data.retain(|char| *char != 0);
    new_data
}

fn pow2u8(b: u8) -> u8 {
    1 << b
}

fn encode(data: &Vec<u8>, path: &str) {
    let seed = rand::thread_rng().gen_range(0..100) % 255 as u8;
    let mut r = StdRng::seed_from_u64(seed as u64);
    let mut new_max: u64 = 2 * pow2u8(r.gen::<u8>() % 3) as u64;

    let mut new_data: Vec<u8> = vec![0; data.len()];
    let mut counter = 0;
    let mut start_index = 0;

    for char in data {
        if counter == new_max {
            start_index += counter;
            counter = 0;
            new_max = 2 * pow2u8(r.gen::<u8>() % 3) as u64;
        }

        if let Some(size) = ((start_index + new_max) as usize).checked_sub(new_data.len()) {
            new_data.extend(vec![0; size]);
        }

        if counter < new_max / 2 {
            new_data[(start_index + counter + new_max / 2) as usize] = *char;
        } else if counter >= new_max / 2 {
            new_data[(start_index + counter - new_max / 2) as usize] = *char;
        }

        counter += 1;
    }

    if counter != new_max {
        while counter < new_max {
            if counter < new_max / 2 {
                new_data[(start_index + counter + new_max / 2) as usize] = 0x0u8;
            } else if counter >= new_max / 2 {
                new_data[(start_index + counter - new_max / 2) as usize] = 0x0u8;
            }
            counter += 1;
        }
    }

    new_data.push(0x4u8);
    new_data.push(seed);

    fs::write(path, new_data).expect("Failed to write.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file argument.");
        exit(1);
    }

    let contents = decode(&args[1]);
    println!("{}", String::from_utf8(contents.clone()).unwrap());

    encode(&contents, &args[1]);
    exit(0);
}
