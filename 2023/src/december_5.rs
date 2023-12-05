pub fn part_1(input: &str) -> i64 {
    let categories = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = to_seeds(categories[0]);
    let seed_to_soil_map = to_maps(categories[1]);
    let soil_to_fertilizer_map = to_maps(categories[2]);
    let fertilizer_to_water_map = to_maps(categories[3]);
    let water_to_light_map = to_maps(categories[4]);
    let light_to_temperature_map = to_maps(categories[5]);
    let temperature_to_humidity_map = to_maps(categories[6]);
    let humidity_to_location_map = to_maps(categories[7]);

    seeds.iter()
        .map(|seed| {
            let mut destination = *seed;
            destination = seed_to_soil_map.map_to_destination(destination);
            destination = soil_to_fertilizer_map.map_to_destination(destination);
            destination = fertilizer_to_water_map.map_to_destination(destination);
            destination = water_to_light_map.map_to_destination(destination);
            destination = light_to_temperature_map.map_to_destination(destination);
            destination = temperature_to_humidity_map.map_to_destination(destination);
            destination = humidity_to_location_map.map_to_destination(destination);
            destination
        })
        .min().unwrap()
}

pub fn part_2(_input: &str) -> i64 {

    0
}

fn to_seeds(input: &str) -> Vec<i64> {
    input.split_once(':').unwrap().1
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap_or_else(|_| panic!("Could not parse seed: {}", seed)))
        .collect()
}

struct Map {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl Map {
    fn in_source_range(&self, source: i64) -> bool {
        source >= self.source_range_start && source < self.source_range_start + self.range_length
    }
    fn map_to_destination_range(&self, source: i64) -> i64 {
        source - self.source_range_start + self.destination_range_start
    }
}

trait DestinationCategory {
    fn map_to_destination(&self, source: i64) -> i64;
}

impl DestinationCategory for Vec<Map> {
    fn map_to_destination(&self, source: i64) -> i64 {
        if let Some(map) = self.iter().rfind(|map| map.in_source_range(source)) {
            map.map_to_destination_range(source)
        } else {
            source
        }
    }
}

fn to_maps(input: &str) -> Vec<Map> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split_whitespace();
            Map {
                destination_range_start: parts.next().unwrap().parse::<i64>().unwrap(),
                source_range_start: parts.next().unwrap().parse::<i64>().unwrap(),
                range_length: parts.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_1(input), 35)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2(input), 0)
}