use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut seeds = Vec::new();

    let mut seed_soil_data_map = DataMap {
        mappings: Vec::new(),
    };

    let mut soil_fertilizer_data_map = DataMap {
        mappings: Vec::new(),
    };

    let mut fertilizer_water_data_map = DataMap {
        mappings: Vec::new(),
    };

    let mut water_light_data_map = DataMap {
        mappings: Vec::new(),
    };

    let mut light_temperature_data_map = DataMap {
        mappings: Vec::new(),
    };

    let mut temperature_humidity_data_map = DataMap {
        mappings: Vec::new(),
    };

    let mut humidity_location_data_map = DataMap {
        mappings: Vec::new(),
    };

    // open a file named input.txt to read line-by-line
    let file = File::open("../input.txt").expect("File not found!");
    let reader = BufReader::new(file);
    let mut phase = 1;
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        //parse ../input.txt into maps by section in the file
        match phase {
            //read in seeds
            1 => {
                println!("{}", line);
                //if line is newline only, move to next phase
                if line == "" {
                    phase = 2;
                } else {
                    //split line into a vector of strings, and remove first value
                    let mut split = line.split(" ");
                    split.next();
                    split.for_each(|seed| {
                        seeds.push(seed.parse::<i64>().unwrap());
                    });
                }
            }
            //read in seed-to-soil map
            2 => {
                if line == "" {
                    phase = 3;
                } else if !line.contains("map") {
                    //create a new mapping and add to seed_soil_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    seed_soil_data_map.mappings.push(mapping);
                }
            }
            3 => {
                //do something
                if line == "" {
                    phase = 4;
                } else if !line.contains("map") {
                    //create a new mapping and add to soil_fertilizer_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    soil_fertilizer_data_map.mappings.push(mapping);
                }
            }
            4 => {
                //do something
                if line == "" {
                    phase = 5;
                } else if !line.contains("map") {
                    //create a new mapping and add to fertilizer_water_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    fertilizer_water_data_map.mappings.push(mapping);
                }
            }
            5 => {
                //do something
                if line == "" {
                    phase = 6;
                } else if !line.contains("map") {
                    //create a new mapping and add to water_light_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    water_light_data_map.mappings.push(mapping);
                }
            }
            6 => {
                //do something
                if line == "" {
                    phase = 7;
                } else if !line.contains("map") {
                    //create a new mapping and add to light_temperature_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    light_temperature_data_map.mappings.push(mapping);
                }
            }
            7 => {
                //do something
                if line == "" {
                    phase = 8;
                } else if !line.contains("map") {
                    //create a new mapping and add to temperature_humidity_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    temperature_humidity_data_map.mappings.push(mapping);
                }
            }
            8 => {
                if !line.contains("map") {
                    //create a new mapping and add to humidity_location_data_map
                    let mut split = line.split(" ");
                    let dest_start = split.next().unwrap().parse::<i64>().unwrap();
                    let source_start = split.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split.next().unwrap().parse::<i64>().unwrap();
                    let mapping = Mapping {
                        dest_start,
                        source_start,
                        range_length,
                    };
                    humidity_location_data_map.mappings.push(mapping);
                }
            }
            _ => {
                println!("Error: Phase not found!");
            }
        }
    });

    let mut find_seed_locations = Vec::new();

    //for each seed, map it through the data maps to get the final location
    seeds.iter().for_each(|seed| {
        let mut location = seed_soil_data_map.convert(*seed);
        location = soil_fertilizer_data_map.convert(location);
        location = fertilizer_water_data_map.convert(location);
        location = water_light_data_map.convert(location);
        location = light_temperature_data_map.convert(location);
        location = temperature_humidity_data_map.convert(location);
        location = humidity_location_data_map.convert(location);
        find_seed_locations.push(location);
    });

    let lowest_number = find_seed_locations.iter().min();
    println!("Lowest number: {}", lowest_number.unwrap());
}

pub struct Mapping {
    pub dest_start: i64,
    pub source_start: i64,
    pub range_length: i64,
}

impl Mapping {
    pub fn contains(&self, value: i64) -> bool {
        if value >= self.source_start && value <= self.source_start + self.range_length {
            return true;
        }
        return false;
    }

    pub fn get_output_val(&self, value: i64) -> i64 {
        if self.contains(value) {
            return value - (self.source_start - self.dest_start);
        }
        return value;
    }
}

pub struct DataMap {
    pub mappings: Vec<Mapping>,
}

impl DataMap {
    //if value is in range of any mapping, return mapped value, else return value
    pub fn convert(&self, value: i64) -> i64 {
        for mapping in self.mappings.iter() {
            if mapping.contains(value) {
                return mapping.get_output_val(value);
            }
        }
        return value;
    }
}