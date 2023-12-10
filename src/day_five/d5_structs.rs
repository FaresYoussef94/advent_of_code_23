
pub const SEEDS:&str="seeds";
pub const SEED_TO_SOIL:&str="seed-to-soil map:";
pub const SOIL_TO_FERTILIZER:&str="soil-to-fertilizer map:";
pub const FERTILIZER_TO_WATER:&str="fertilizer-to-water map:";
pub const WATER_TO_LIGTH:&str="water-to-light map:";
pub const LIGHT_TO_TEMPERATURE:&str="light-to-temperature map:";
pub const TEMPERATURE_TO_HUMIDITY:&str="temperature-to-humidity map:";
pub const HUMIDITY_TO_LOCATION:&str="humidity-to-location map:";

#[derive(Default, Debug)]
pub struct Almanac{
    pub seeds:Vec<u32>, 
    pub seed_to_soil: Vec<Map>,
    pub soil_to_fertilizer: Vec<Map>,
    pub fertilizer_to_water:Vec<Map>,
    pub water_to_light:Vec<Map>,
    pub light_to_temperature:Vec<Map>,
    pub temperature_to_humidity:Vec<Map>,
    pub humidity_to_location:Vec<Map>,
}


#[derive(Default, Debug)]
pub struct Map {
    pub source: u32,
    pub destination: u32,
    pub count: u32,
}


pub struct SeedRange{
    pub i: u64,
    pub j: u64,
}
