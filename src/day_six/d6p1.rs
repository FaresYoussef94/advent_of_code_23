use array2d::Array2D;
use crate::{day_six::d6_structs::TimeDistance, utils::file_reader::read_lines};

const MAXIMUM_RACE_TIME:usize = 86;

pub fn d6p1() {
    let arr = build_dp_array();
    let time_distance = get_time_and_distance();
    let result = record_product(arr, time_distance);
    println!("D6P1 result: {}", result);
}

pub fn build_dp_array () -> Array2D<usize> {
    let mut arr:Array2D<usize> = Array2D::filled_with(0, MAXIMUM_RACE_TIME+1, MAXIMUM_RACE_TIME+1);

    for i in 1..=MAXIMUM_RACE_TIME {
        for j in 1..=MAXIMUM_RACE_TIME {
            if i <= j {
                continue;
            }
            let prior_value = arr.get(i-1, j).unwrap();
            let _ = arr.set(i, j, prior_value + j);
        }
    }
     arr
}

fn get_time_and_distance () -> TimeDistance {
    let mut time:Vec<usize> = Vec::new();
    let mut distance:Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines("./resources/d6.txt") {
        for line in lines {
            if let Ok(entry) = line {
                if entry.contains("Time") {
                    for item in entry.split(" ") {
                        match item.parse::<usize>().ok() {
                            Some(parsed_item) => time.push(parsed_item),
                            None => continue,
                        }
                    }
                } else if entry.contains("Distance") {
                    for item in entry.split(" ") {
                        match item.parse::<usize>().ok() {
                            Some(parsed_item) => distance.push(parsed_item),
                            None => continue,
                        }
                    }


                }
            }
        }
    }


    TimeDistance{ time, distance}
}

fn record_product(arr: Array2D<usize>, time_distance: TimeDistance) -> u32 {
    let mut result: u32 = 1;

    for i in 0..time_distance.distance.len() {
        let mut local_result = 0;
        for j in 1..time_distance.time[i] {
            let current_reading = *arr.get(time_distance.time[i], j).unwrap();
            let aim_distance = time_distance.distance[i];
            if current_reading == 0 {
                continue;
            } else if current_reading > aim_distance {
                local_result+=1;
            }
        }
        result *= local_result;
    }

   result 
} 

