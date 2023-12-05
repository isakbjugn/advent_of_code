pub fn part_1(input: &str) -> i64 {
    let (seeds_str, categories_str) = input.split_once("\n\n").unwrap();
    let seeds = to_seeds(seeds_str);
    let categories = to_categories(categories_str);
    find_closest_location(seeds, &categories)
}

pub fn part_2(input: &str) -> i64 {
    let (seeds_str, categories_str) = input.split_once("\n\n").unwrap();
    let seeds = to_seeds_from_range(seeds_str);
    let categories = to_categories(categories_str);

    find_closest_location(seeds, &categories)
}

fn find_closest_location(seeds: Vec<i64>, categories: &Vec<Vec<Map>>) -> i64 {
    seeds.iter().map(|seed| {
        let mut destination = *seed;
        for category in categories {
            destination = category.map_to_destination(destination);
        }
        destination
    }).min().unwrap()
}

fn to_categories(categories_str: &str) -> Vec<Vec<Map>> {
    categories_str.split("\n\n").map(to_maps).collect()
}

fn to_seeds(input: &str) -> Vec<i64> {
    input.split_once(':').unwrap().1
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap_or_else(|_| panic!("Could not parse seed: {}", seed)))
        .collect()
}

fn to_seeds_from_range(input: &str) -> Vec<i64> {
    let mut parts = input.split_once(':').unwrap().1.split_whitespace();
    let mut seeds = Vec::<i64>::new();

    while let Some(start_str) = parts.next() {
        let end_str = parts.next().unwrap_or("");
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str.parse::<i64>().unwrap();
        seeds.append(&mut (start..start+end).collect());
    }
    seeds
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
fn sample_input_part_2() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2(input), 46)
}
