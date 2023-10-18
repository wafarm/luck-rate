use std::io::{stdin, stdout, Write};

use chrono::{Datelike, Local};

use crate::random::Random;

mod random;

fn main() {
    let date = Local::now();
    let year = date.year() as u64;
    let month = date.month0() as u64;
    let day = date.day() as u64;

    let seed_base = year + month + day;
    let mut input = String::new();

    print!("QQ number: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let qq = input.trim().parse::<u64>().unwrap();

    let seed = (seed_base + qq) as i64;

    let mut random = Random::new(seed);
    let result = random.bounded_next_i32(0, 101);

    println!("{result}");
}
