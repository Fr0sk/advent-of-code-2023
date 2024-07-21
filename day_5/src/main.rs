const INPUT_DATA: &str = include_str!("input.txt");
const EXAMPLE_DATA: &str = include_str!("example.txt");

#[derive(Debug, Default, Clone, Copy)]
struct Map {
    source: u64,
    dest: u64,
    range: u64
}

#[derive(Debug, Default, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<Map>>,
    /*seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,*/
}

impl Map {
    fn parse(data: &str) -> Self {
        let values: Vec<u64> = data.splitn(3, " ")
            .map(|x| x.parse().expect("Number"))
            .collect();
        
        Self { 
            source: values[1], 
            dest: values[0], 
            range: values[2] 
        }
    }

    fn contains(&self, value: u64) -> bool {
        self.source <= value && self.source + self.range > value
    }

    fn find_destination(&self, value: u64) -> Option<u64> {
        if self.contains(value) {
            return Some(self.dest + value - self.source);
        }
        
        None
    }
}

impl Almanac {
    fn parse(data: &str) -> Self {
        let mut almanac = Almanac::default();

        for line in data.lines() {
            if line.starts_with("seeds:") {
                let (_, values) = line.split_once(":").expect("Malformed seeds");
                for val in values.split_whitespace() {
                    almanac.seeds.push(val.parse().expect(&format!("Cannot parse number ({val})")));
                }
            }
            else if line.contains(":") {
                almanac.maps.push(Vec::new())
            } else if !line.is_empty() {
                almanac.maps.last_mut().expect("Should have a Vec<Map>").push(Map::parse(line))
            }
        }

        almanac
    }

    fn process_seed(&self, seed: u64) -> u64 {
        let mut next = seed;

        for map in &self.maps {
            let map_opt = map.iter().find(|m| m.contains(next));
            if let Some(m) = map_opt {
                next = m.find_destination(next).unwrap_or(next);
            }
        }

        next
    }
}

fn main() {
    println!("Part 1: {}", part_1(INPUT_DATA));
    println!("Part 2: {}", part_2(EXAMPLE_DATA));
}

fn part_1(data: &str) -> u64 {
    let almanac = Almanac::parse(data);
    let mut min = u64::MAX;
    
    for seed in &almanac.seeds {
        let location = almanac.process_seed(*seed);
        if location < min {
            min = location;
        }
    }
    
    min
}

fn part_2(data: &str) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..900000000u64 {
        sum += 1;
    }
    sum
}