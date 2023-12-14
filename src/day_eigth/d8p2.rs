use crate::utils::file_reader::read_lines;
use crate::day_eigth::d8_structs::NavMap;
use std::collections::HashMap;
use std::usize;
use num_integer::lcm;

pub fn d8p2 () {
    let (nav, start_points, map) = build_map();
    let result = nav_map(nav, &start_points, map);

    println!("D8P2 result: {:?}", result.iter().fold(1, |acc, &curr| lcm(acc, curr)));
}

fn build_map () -> (Vec<char>, Vec<String>, HashMap<String, NavMap> ) {
    let mut start_points:Vec<String> = Vec::new();
    let mut nav:Vec<char> = Vec::new();
    let mut map:HashMap<String, NavMap> = HashMap::new(); 
    
    let mut is_first_line = true;
    
    if let Ok(lines) = read_lines("./resources/d8.txt") {
        for line in lines{
            if let Ok(entry) = line {
                if is_first_line {
                    nav = entry.chars().collect();
                    is_first_line = false;
                    continue;
                }

                if entry.trim().is_empty() {
                    continue;
                }

                let key = &entry[..=2];
                let left_value = &entry[7..=9];
                let right_value = &entry[12..=14];

                map.insert(String::from(key), NavMap { left: String::from(left_value), right: String::from(right_value) });
            
                if key.ends_with("A") {
                    start_points.push(key.to_string());
                } 
            }
        }
    }

    (nav, start_points, map)
}


fn nav_map(nav:Vec<char>, start_points:&Vec<String>, map:HashMap<String, NavMap>) -> Vec<usize> {
    let mut results:Vec<usize> = Vec::new(); 
    
    for i in 0..start_points.len() {
        let mut current_point:String = start_points.get(i).unwrap().clone().to_string();
        let mut current_ending_points:Vec<usize> = Vec::new();

        let mut local_iterator:usize = 0;
        let mut j:usize = 0;
        while !is_pattern_formed(&current_ending_points) {
            if has_map_ended(&current_point) {
                current_ending_points.push(j);
            }

            let direction = nav.get(local_iterator).unwrap().to_string();
            let left = &map.get(&current_point).unwrap().left;
            let right = &map.get(&current_point).unwrap().right;

            if direction == "L" {
                current_point = left.to_string(); 
            } else {
                current_point = right.to_string(); 
            }

            local_iterator+=1;
            j+=1;

            if local_iterator == nav.len() {
                local_iterator = 0;
            }

        }
        results.push(get_pattern(&current_ending_points));
    }

    results
}

fn is_pattern_formed(points: &Vec<usize>) -> bool {
    if points.len() < 3 {
        return false;
    }

    let point_one = points.get(points.len() -3).unwrap();
    let point_two = points.get(points.len() -2).unwrap();
    let point_three = points.get(points.len() -1).unwrap();


    if point_three - point_two == point_two - point_one {
        return true;
    }

    false
}

fn get_pattern(points: &Vec<usize>) -> usize {
   points.get(points.len() - 1).unwrap() - points.get(points.len() - 2).unwrap()
}

fn has_map_ended(current_point:&String) -> bool {
    if !current_point.ends_with("Z") {
        return false;
    }
    true
}
