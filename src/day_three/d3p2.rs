use array2d::Array2D;
use crate::day_three::d3p1::{get_2d_array, is_char_num};

const GEAR_CHAR:char = '*';

pub fn d3p2() {
    let arr = get_2d_array();
    let result = get_gear_ratio(arr);

    println!("D3p2 result: {}", result);
}

fn get_gear_ratio(arr: Array2D<char>) -> u32 {
    let mut result: u32 = 0;

    for i in 0..140 {
        let mut j = 0;
        while j < 140 {
            if is_char_gear(&arr, i, j) {
                result += get_adjacent_ratio(&arr, i, j); 
            }
            j += 1;
        }
    }

    result
}

fn is_char_gear(arr: &Array2D<char>, i:usize, j:usize) -> bool {
   *arr.get(i,j).unwrap() == GEAR_CHAR  
}

fn get_adjacent_ratio(arr: &Array2D<char>, i:usize, j:usize) -> u32 {
    let mut _flag = false;

    let mut first_ratio:u32 = 0;
    let mut second_ratio:u32 = 0;

    for a in i-1..=i+1 {
        for b in j-1..=j+1 {
            if is_char_num_dup(arr, a.try_into().unwrap(), b.try_into().unwrap()) && !_flag {
                let current_number = get_number(arr, a, b);
                if first_ratio == 0 {
                    first_ratio = current_number;
                }else {
                    second_ratio = current_number;
                }
                _flag = true;
            } else if !is_char_num(arr, a, b) {
                _flag = false;
            }
        } 
        _flag = false;
    }


    return first_ratio * second_ratio; 
}

fn get_number(arr: &Array2D<char>, i:usize, j:usize) -> u32{
    let mut num: u32 = 0;
    let mut k = j;

    while k > 0 && is_char_num_dup(&arr, i.try_into().unwrap(), (k-1).try_into().unwrap()) {
        k-=1;
    }

    while is_char_num_dup(&arr, i.try_into().unwrap(), k.try_into().unwrap()){
        num = num * 10 + arr.get(i,k).unwrap().to_digit(10).unwrap();
        k+=1;
    }

    num
}

fn is_char_num_dup(arr:&Array2D<char>, i:i32, j:i32) -> bool {
    i >= 0 && i < 140 && j >= 0 && j < 140 && arr.get(i.try_into().unwrap(),j.try_into().unwrap()).unwrap().is_digit(10)
}

