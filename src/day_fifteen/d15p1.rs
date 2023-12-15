use std::{u32, str::Chars};
use crate::utils::file_reader::read_lines;

pub fn d15p1() {
    let result = get_hash_sum();

    println!("D15P1 result: {}", result);
}


fn get_hash_sum() -> u32{
    let mut sum:u32 = 0; 

    if let Ok(lines) = read_lines("./resources/d15.txt") {
        for line in lines {
            if let Ok(entry) = line {
                println!("Line: {}", entry);
                for chars in entry.split(",") {
                    println!("Chars: {}", chars);
                    sum += get_hash(&chars.chars());
                }
            }
        }
    }

    sum
}

fn get_hash(chars: &Chars) -> u32 {
    let mut hash_value:u32 = 0;

    for ch in chars.clone().into_iter() {
        hash_value += ch as u32;
        hash_value *= 17;
        hash_value %= 256;
    }

    println!("{:?} {}", chars, hash_value);
    hash_value
}

