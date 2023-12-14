use crate::utils::file_reader::read_lines;
use array2d::Array2D;


pub fn d9p1 () {
    let arr = build_map();
    let result = get_result(&arr);

    println!("D9P1 result: {}", result);
}


pub fn build_map () -> Array2D<i32> {
    let mut nums: Array2D<i32> = Array2D::filled_with(0, 200, 21);

    if let Ok(lines) = read_lines("./resources/d9.txt") {
        let mut i:usize = 0;
        for line in lines{
            if let Ok(entry) = line {
                let mut j:usize = 0;
              
                for number in entry.split(" ") {
                    if number.trim().is_empty() {
                        continue;
                    }

                    let parsed_value:i32 = number.parse::<i32>().unwrap();
                    let _ = nums.set(i, j, parsed_value);
                    j+=1;
                }

                i+=1;
            }
        }
    }

    nums
}

fn get_result(arr:&Array2D<i32>) -> i32 {
    let mut result:i32 = 0;
    for row in arr.as_rows() {
        let local_rsult= recursive(&row);
        result += local_rsult;
    }
    result
}

fn recursive(diff_arr:&Vec<i32>) -> i32 {
    if is_base_state(&diff_arr) {
        return *diff_arr.get(0).unwrap();
    } 
    
    let mut level_diff_arr:Vec<i32> = Vec::new();
    for i in 1..diff_arr.len() {
        level_diff_arr.push(diff_arr.get(i).unwrap() - diff_arr.get(i-1).unwrap());
    }

    return diff_arr.get(diff_arr.len()-1).unwrap() + recursive(&level_diff_arr);
}


fn is_base_state(diff_arr:&Vec<i32>) -> bool {
    for i in 1..diff_arr.len() {
        if diff_arr.get(i).unwrap() != diff_arr.get(i-1).unwrap() {
            return false;
        }
    }
    true
}
