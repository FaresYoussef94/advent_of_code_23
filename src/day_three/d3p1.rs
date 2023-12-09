use crate::utils::file_reader::read_lines;
use array2d::Array2D;


const DOT_CHAR: char = '.';

pub fn d3p1() {
    let arr = get_2d_array();
    let result = get_engine_model(arr);

    println!("D3P1 result: {}", result);
}

pub fn get_2d_array() -> Array2D<char> {
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut arr = Array2D::filled_with('.', 140, 140);

    if let Ok(lines) = read_lines("./resources/d3p1.txt") {
        for line in lines {
            if let Ok(entry) = line {
                for c in entry.chars() {
                    let _ = arr.set(j, i, c);
                    i+=1;
                }
            }
            i=0;
            j+=1;
        }
    }

    arr
}

fn get_engine_model(arr: Array2D<char>) -> u32 {
    let mut result: u32 = 0;

    for i in 0..140 {
        let mut j = 0;
        while j < 140 {
            if is_char_num(&arr, i, j) {
                let mut current_num: u32 = 0;
                let mut k: usize = j;
                while is_char_num(&arr, i, k) {
                   current_num = 10 * current_num + arr.get(i, k).unwrap().to_digit(10).unwrap(); 
                    k+=1; 
                }
                k-=1;
                if is_number_adjacent_to_symbol(&arr, i.try_into().unwrap(), j.try_into().unwrap(), k.try_into().unwrap()) {
                    result += current_num;
                }
                j = k;
            } 
            j += 1;
        }
    }

    result
}

pub fn is_char_num(arr: &Array2D<char>, i:usize, j: usize) -> bool {
    j < 140 && arr.get(i,j).unwrap().is_digit(10)
}

fn is_number_adjacent_to_symbol(arr: &Array2D<char>, i:i32, j:i32, k:i32) -> bool {
    for a in i-1..=i+1 {
        for b in j-1..=k+1 {
            if a < 0 || a > 139 || b < 0 || b > 139 || (a == i && (b >= j && b <= k)) {
                continue;
            }
            if is_char_num(arr, a.try_into().unwrap(), b.try_into().unwrap()) {
                continue;
            }
            let adj_char = arr.get(a.try_into().unwrap(), b.try_into().unwrap()).unwrap();  
            if *adj_char != DOT_CHAR {
                return true
            }
        }
    }    

    false
}
