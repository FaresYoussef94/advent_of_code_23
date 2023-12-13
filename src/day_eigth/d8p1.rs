use crate::utils::file_reader::read_lines;
use crate::day_eigth::d8_structs::NavMap;
use std::collections::HashMap;

pub fn d8p1 () {
    let (nav, map) = build_map();
    let result = nav_map(nav, map);

    println!("D8P1 result: {}", result);
}

fn build_map () -> (Vec<char>, HashMap<String, NavMap> ) {
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
            }
        }
    }

    (nav, map)
}

fn nav_map(nav:Vec<char>, map:HashMap<String, NavMap>) -> u32 {
    let mut result:u32 = 0;
    let mut i:usize = 0;
    let mut current_point:String = String::from("AAA");
    
    while current_point != "ZZZ" {
        let direction = nav.get(i).unwrap();
        let left = &map.get(&current_point).unwrap().left;
        let right = &map.get(&current_point).unwrap().right;
        
        if direction.to_string() == "L" {
            current_point = left.to_string();
        } else {
            current_point = right.to_string();
        }

        result += 1;
        i+=1;
        
        if i == nav.len() {
            i = 0; 
        }
    }

    result
}
