use crate::{day_six::d6_structs::TimeDistance, utils::file_reader::read_lines};


pub fn d6p2() {
    let time_distance = get_time_and_distance();
    let result = record_product(time_distance);
    println!("D6P2 result: {}", result);
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

fn record_product(time_distance: TimeDistance) -> u32 {
    let mut result: u32 = 0;
    let time = time_distance.time.get(0).unwrap();
    let distance = time_distance.distance.get(0).unwrap();

    for i in 0..=*time {
        let distance_max = (time - i)*i;
        if distance_max >= *distance {
            result +=1;
        }
    }

   result 
} 

