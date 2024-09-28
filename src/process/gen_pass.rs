use rand::seq::SliceRandom;
use rand::Rng;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"23456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
    length: u8,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut charset = Vec::new();

    if uppercase {
        charset.extend_from_slice(UPPER);
        password.push(UPPER[rng.gen_range(0..UPPER.len())]);
    }
    if lowercase {
        charset.extend_from_slice(LOWER);
        password.push(LOWER[rng.gen_range(0..LOWER.len())]);
    }
    if number {
        charset.extend_from_slice(NUMBER);
        password.push(NUMBER[rng.gen_range(0..NUMBER.len())]);
    }
    if symbol {
        charset.extend_from_slice(SYMBOL);
        password.push(SYMBOL[rng.gen_range(0..SYMBOL.len())]);
    }

    for _ in 0..(length - password.len() as u8) {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset[idx]);
    }

    //　TODO: make sure the password contains at least one of each type
    password.shuffle(&mut rng);

    println!("{}", String::from_utf8(password)?);
    Ok(())
}
