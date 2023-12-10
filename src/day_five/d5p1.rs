use crate::utils::file_reader::read_lines;
use crate::day_five::d5_structs::*;

pub fn d5p1() {
    let almanac = build_almanac();
    let result = get_lowest_location(&almanac);
    println!("D5P1 result: {}", result);
}

pub fn build_almanac() -> Almanac {
    let mut seeds:Vec<u32>  = Vec::new();
    let mut seed_to_soil: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer: Vec<Map> = Vec::new();
    let mut fertilizer_to_water: Vec<Map> = Vec::new();
    let mut water_to_light: Vec<Map> = Vec::new();
    let mut light_to_temperature:Vec<Map> = Vec::new();
    let mut temperature_to_humidity:Vec<Map> = Vec::new();
    let mut humidity_to_location: Vec<Map> = Vec::new();

    let mut current_map = &mut seed_to_soil;

    if let Ok(lines) = read_lines("./resources/d5p1.txt") {
        for line in lines{
            if let Ok(entry) = line {
                if entry.trim().is_empty(){
                    continue;
                } else if entry.contains(SEEDS){
                    let seeds_entry = entry.split(" ");
                    for seed_item in seeds_entry {
                        match seed_item.parse::<u32>().ok(){
                            Some(item) => seeds.push(item),
                            None => continue,
                        }
                    }
                    continue;
                } else if entry.contains(SEED_TO_SOIL) {
                    current_map = &mut seed_to_soil;
                } else if entry.contains(SOIL_TO_FERTILIZER) {
                    current_map = &mut soil_to_fertilizer;
                } else if entry.contains(FERTILIZER_TO_WATER) {
                    current_map = &mut fertilizer_to_water;
                } else if entry.contains(WATER_TO_LIGTH) {
                    current_map = &mut water_to_light;
                } else if entry.contains(LIGHT_TO_TEMPERATURE) {
                    current_map = &mut light_to_temperature;
                } else if entry.contains(TEMPERATURE_TO_HUMIDITY) {
                    current_map = &mut temperature_to_humidity;
                } else if entry.contains(HUMIDITY_TO_LOCATION) {
                    current_map = &mut humidity_to_location;
                } else {
                    let parts = entry.split(" ");
                    let source:u32 = parts.clone().nth(1).unwrap().parse::<u32>().unwrap(); 
                    let destination:u32 = parts.clone().nth(0).unwrap().parse::<u32>().unwrap(); 
                    let count:u32 = parts.clone().nth(2).unwrap().parse::<u32>().unwrap();

                    current_map.push(Map { source, destination, count});
                }
            }
        }
    }

    Almanac { 
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn get_lowest_location(almanac: &Almanac) -> u64 {
    let mut result = u64::MAX;

    for seed in almanac.seeds.iter() {
        let soil = get_mapping_value(*seed as u64, &almanac.seed_to_soil); 
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

pub fn get_mapping_value(num:u64, mappings: &Vec<Map>) -> u64 {
   for map_item in mappings {
        if is_within_range(num.into(), map_item.source.into(), map_item.count.into()) {
            return get_precise_destination(num, map_item.source.into(), map_item.destination.into());
        }
    }
    num
}

pub fn is_within_range(num: u64, source:u64, count:u64) -> bool {
   num >= source && num <= source + count -1
}

pub fn get_precise_destination(num:u64, source:u64, destination:u64) -> u64 {
    destination + (num - source)
}

