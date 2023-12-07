use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn d1p1() {
    let mut result: u32 = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./resources/d1p1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let (first_digit, last_digit ) = get_first_and_last_digits(entry);
                result += first_digit*10 +last_digit;
            }
        }
    }

    println!("Final result: {}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_first_and_last_digits(entry: String) -> (u32, u32){
    let mut first_digit: u32 = 420420;
    let mut last_digit: u32 = 6969;

    let char_vec:Vec<char> = entry.chars().collect();

    for c in char_vec.iter(){
        if c.to_digit(10).is_some(){
            first_digit = c.to_digit(10).unwrap(); 
            break;
        }
    }

    for c in char_vec.iter().rev(){
        if c.to_digit(10).is_some(){
            last_digit = c.to_digit(10).unwrap(); 
            break;
        }

    }

    return (first_digit, last_digit)
}
