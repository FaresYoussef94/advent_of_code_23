use std::collections::HashSet;

use crate::utils::file_reader::read_lines;
use array2d::Array2D;

const SIZE:usize=13;

pub fn d17p1(){
    let arr = build_array();
    let mut visited_nodes:HashSet<String> = HashSet::new();
    let exp_r = find_least_heat('r', 0, 1, 0, &arr, &mut visited_nodes);
    let exp_d = find_least_heat('d', 0, 0, 1, &arr, &mut visited_nodes);
    let result = u32::min(exp_r, exp_d);

    println!("D17P1 result: {}", result);
}

pub fn build_array() -> Array2D<usize> {
    let mut arr:Array2D<usize> = Array2D::filled_with(0, SIZE, SIZE);

    if let Ok(lines) = read_lines("./resources/d17.txt") {
        let mut i:usize = 0;
        for line in lines{
            if let Ok(entry) = line {
                let mut j:usize = 0;
                for ch in entry.chars() {
                    let _ = arr.set(i, j, ch.to_string().parse::<usize>().unwrap());
                    j+=1;
                }
            }
        i+=1;
        }
    }
    arr
}


fn find_least_heat(prev_direction:char, direction_count:usize, i:isize, j:isize, arr: &Array2D<usize>, visited_nodes:&mut HashSet<String>) -> u32 {
    println!("{} {} {} {}",prev_direction, direction_count, i, j);
    let mut node:String = format!("{}-{}", i.to_string(), j.to_string());
    if i == SIZE as isize - 1 && j == SIZE as isize - 1 {
        return *arr.get(i as usize, j as usize).unwrap() as u32;
    } else if i == SIZE as isize || j == SIZE  as isize {
        return u32::MAX;
    } else if i == 0 && j == 0 {
        return u32::MAX;
    } else if i < 0 || j < 0 {
        return u32::MAX;
    } else if visited_nodes.contains(&node) {
        return u32::MAX;
    } 

    visited_nodes.insert(node);

    let mut go_l = u32::MAX;
    let mut go_r = u32::MAX;
    let mut go_u = u32::MAX;
    let mut go_d = u32::MAX;

    match prev_direction {
       'r' => {
                if direction_count < 2 {
                    go_r = find_least_heat('r', direction_count+1, i+1, j, arr, visited_nodes)
                }
                if j > 0 {
                    go_u = find_least_heat('u', 0, i, j-1, arr, visited_nodes);
                }
                go_d = find_least_heat('d', 0, i, j+1, arr, visited_nodes);
        },
       'd' => {
                if direction_count < 2 {
                    go_d = find_least_heat('d', direction_count+1, i, j+1, arr, visited_nodes)
                }
                if i > 0 {
                    go_l = find_least_heat('l', 0, i-1, j, arr, visited_nodes);
                }
                go_r = find_least_heat('r', 0, i+1, j, arr, visited_nodes);
        },
      'l' => {
                if direction_count < 2 {
                    go_l = find_least_heat('l', direction_count+1, i-1, j, arr, visited_nodes)
                }
                if j > 0 {
                    go_u = find_least_heat('u', 0, i, j-1, arr, visited_nodes);
                }
                go_d = find_least_heat('d', 0, i, j+1, arr, visited_nodes);
        },
        'u' => {
                if direction_count < 2 {
                    go_u = find_least_heat('u', direction_count+1, i, j-1, arr, visited_nodes)
                }
                if i > 0 {
                    go_l = find_least_heat('l', 0, i-1, j, arr, visited_nodes);
                }
                go_r = find_least_heat('r', 0, i+1, j, arr, visited_nodes);
        },
        _ => {},
    }



    let next_min =  u32::min(go_d, u32::min(go_u,u32::min(go_l, go_r)));
    if next_min == u32::MAX {
        return u32::MAX;
    }

    return *arr.get(i.try_into().unwrap(), j.try_into().unwrap()).unwrap() as u32 + next_min;
}
