use crate::day_five::d5p1::*;
use crate::day_five::d5_structs::{Almanac, SeedRange};
use rayon::prelude::*;
use std::time::Instant;

pub fn d5p2 () {
    let  almanac = build_almanac();
    let seed_range = get_min_max_seed_range(&almanac.seeds);
    let start_time = Instant::now();
    let result:u64 = seed_range.par_iter().map(|chunk| get_lowest_location(chunk, &almanac)).reduce(|| u64::MAX, |min_value, value| u64::min(min_value, value));
    let end_time = Instant::now();
    println!("D5P2 result: {} - took: {}", result, (end_time - start_time).as_secs());

}


fn get_min_max_seed_range(seed: &Vec<u32>) -> Vec<SeedRange> {
    let mut seed_range:Vec<SeedRange> = Vec::new(); 
    let j = seed.len()/2;

    for i in 0..j {
        seed_range.push(SeedRange { i: *seed.get(i*2).unwrap() as u64, j: *seed.get(i*2).unwrap() as u64 + *seed.get(i*2 + 1).unwrap() as u64 });
    }

    seed_range
}


fn get_lowest_location(seed_range: &SeedRange, almanac: &Almanac) -> u64 {
    let mut result = u64::MAX;

    for seed in seed_range.i..=seed_range.j {
        let soil = get_mapping_value(seed, &almanac.seed_to_soil); 
        let fertilizer = get_mapping_value(soil, &almanac.soil_to_fertilizer); 
        let water = get_mapping_value(fertilizer, &almanac.fertilizer_to_water); 
        let light = get_mapping_value(water, &almanac.water_to_light); 
        let temperature = get_mapping_value(light, &almanac.light_to_temperature); 
        let humidity = get_mapping_value(temperature, &almanac.temperature_to_humidity); 
        let location = get_mapping_value(humidity, &almanac.humidity_to_location); 
        result = u64::min(result, location);
    }

    result
}
