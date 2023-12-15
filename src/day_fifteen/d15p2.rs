use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::utils::file_reader::read_lines;

use super::d15p1::get_hash;

pub fn d15p2() {
    let (lens_map, boxes) = parse_input();
    println!("{:?}", lens_map);
    println!("{:?}", boxes);
    let result = calculate_result(lens_map, boxes);

    println!("D15p2 result: {}", result);
}


fn calculate_result(lens_map: HashMap<String, u32>, boxes: HashMap<u32, Vec<String>>) -> u32 {
    let mut sum: u32 = 0;

    for  (box_num, lens) in boxes.iter() {
        let mut i:usize = 1;
        for len_label in lens {
           match lens_map.get(len_label) {
                Some(value) => {
                    if *value == 0 {
                        continue;
                    } else {
                        println!("Multiplying: {} * {} * {}", (box_num+1) , i as u32 , value);
                        sum += (box_num+1) * i as u32 * value;
                        i+=1;
                    }
                },
                None => continue,
            }

        }
    }
    sum
}

fn parse_input() -> (HashMap<String, u32>, HashMap<u32, Vec<String>>) {
    let mut lens_map:HashMap<String, u32> = HashMap::new();
    let mut boxes:HashMap<u32, Vec<String>> = HashMap::new();

    if let Ok(lines) = read_lines("./resources/d15.txt") {
        for line in lines {
            if let Ok(entry) = line {
                for lenses in entry.split(",") {
                    if lenses.contains("-") {
                        handle_removal(lenses[.. lenses.len() - 1].to_string(), &mut lens_map, &mut boxes)
                    } else if lenses.contains("=") {
                        handle_addition(lenses[.. lenses.len() - 2].to_string(), lenses.chars().last().unwrap().to_string().parse::<u32>().unwrap(), &mut lens_map, &mut boxes)

                    }
                }
            }
        }
    }

    (lens_map, boxes)
}

fn handle_addition(key:String, value:u32, lens_map: &mut HashMap<String, u32>, boxes: &mut HashMap<u32, Vec<String>>) {
    match lens_map.entry(key.clone()) {
        Entry::Occupied(mut entry) => {
            *entry.get_mut() = value;
            return;
        },
        Entry::Vacant(entry) => entry.insert(value),
    };

    let hash_value = get_hash(&key.chars()); 

    match boxes.entry(hash_value) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(key)
        },
        Entry::Vacant(entry) => {
            entry.insert(vec![key]);
        },
    };
}

fn handle_removal(key: String, lens_map: &mut HashMap<String, u32>, boxes: &mut HashMap<u32, Vec<String>>) {
    let value = lens_map.remove(&key);
    let hash_value = get_hash(&key.chars()); 

    match value {
        Some(_value) => {
            match boxes.entry(hash_value) {
                Entry::Occupied(mut entry) => {
                    let mut i:usize = 0;
                    for lenses in entry.get_mut() {
                        if lenses.to_string() == key {
                            break;
                        }
                        i+=1;
                    }
                    entry.get_mut().remove(i);
                },
                Entry::Vacant(_entry) => {},
            }
        },
        None => {}
    }
    
}
