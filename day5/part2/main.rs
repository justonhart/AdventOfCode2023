use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut seed_codes = Vec::new();

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
                //if line is newline only, move to next phase
                if line == "" {
                    phase = 2;
                } else {
                    //split line into a vector of strings, and remove first value
                    let split = line.split(" ");
                    
                    //collect the split values into a vector of strings
                    let mut tokens = split.collect::<Vec<&str>>();

                    //remove the first value from the vector
                    tokens = tokens[1..].to_vec();
                    
                    //convert the vector of strings into a vector of i64s
                    let seed_tokens = tokens.iter().map(|token| {
                        token.parse::<i64>().unwrap()
                    }).collect::<Vec<i64>>();
                    
                    for i in (0..seed_tokens.len()).step_by(2) {
                        let seed_code = SeedCode {
                            start_number: seed_tokens[i],
                            count: seed_tokens[i + 1],
                        };
                        seed_codes.push(seed_code);
                    }
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

    let mut count = 0;
    loop {

        //check if cound exists in location map and trace backward to reach an existing seed. If seed exists, break; else increment count
        let current_location = humidity_location_data_map.reverse(count);
        let current_temperature = temperature_humidity_data_map.reverse(current_location);
        let current_light = light_temperature_data_map.reverse(current_temperature);
        let current_water = water_light_data_map.reverse(current_light);
        let current_fertilizer = fertilizer_water_data_map.reverse(current_water);
        let current_soil = soil_fertilizer_data_map.reverse(current_fertilizer);
        let seed_code = seed_soil_data_map.reverse(current_soil);
        
        //check seed_codes for SeedCode with start_number <= seed_code and start_number + count >= seed_code
        let mut found_seed = false;
        for seed in seed_codes.iter() {
            if seed.start_number <= seed_code && seed.start_number + seed.count > seed_code {
                found_seed = true;
                break;
            }
        }
        if found_seed {
            break;
        } else {
            count += 1;
        }
    }

    println!("Lowest number: {}", count);
}

pub struct Mapping {
    pub dest_start: i64,
    pub source_start: i64,
    pub range_length: i64,
}

impl Mapping {
    pub fn contains(&self, value: i64) -> bool {
        if value >= self.source_start && value < self.source_start + self.range_length {
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
    
    pub fn output_in_range(&self, value: i64) -> bool {
        if value >= self.dest_start && value < self.dest_start + self.range_length {
            return true;
        }
        return false;
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
    
    pub fn reverse(&self, value: i64) -> i64 {
        for mapping in self.mappings.iter() {
            if mapping.output_in_range(value) {
                return mapping.source_start + (value - mapping.dest_start);
            }
        }
        return value;
    }
}

pub struct SeedCode {
    pub start_number: i64,
    pub count: i64,
}