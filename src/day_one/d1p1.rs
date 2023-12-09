use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;
use crate::day_one::d1_structs::ValueNIndex;

pub fn d1p1() {
    let mut result: u32 = 0;
    if let Ok(lines) = read_lines("./resources/d1p1.txt") {
        for line in lines {
            if let Ok(entry) = line {
                let (first_digit, last_digit ) = get_first_and_last_digits(entry);
                match first_digit{
                    Some(x) => result+=x.value*10,
                    None => result+=0,
               }
                match last_digit{
                    Some(x) => result+=x.value,
                    None => result+=0,
               }

            }
        }
    }

    println!("D1P2 result: {}", result);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn get_first_and_last_digits(entry: String) -> (Option<ValueNIndex>, Option<ValueNIndex>){
    let mut first_digit: Option<ValueNIndex> = None;
    let mut last_digit: Option<ValueNIndex> = None;

    let char_vec:Chars = entry.chars();
    let rev_char_vec:Chars = char_vec.clone();
    let size:usize = char_vec.clone().count();

    for (i,c) in char_vec.enumerate(){
        if c.to_digit(10).is_some(){
            first_digit = Some(ValueNIndex{
                index: i,
                value: c.to_digit(10).unwrap(),
           }); 
            break;
        }
    }

    for (i,c) in rev_char_vec.rev().enumerate(){
        if c.to_digit(10).is_some(){
            // last_digit = c.to_digit(10); 
            last_digit = Some(ValueNIndex{
                index: size - i -1,
                value: c.to_digit(10).unwrap(),
            });
            break;
        }

    }

    return (first_digit, last_digit)
}
