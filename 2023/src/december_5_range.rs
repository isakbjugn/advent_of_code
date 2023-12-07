use std::collections::HashSet;
use std::iter::{once, zip};
use itertools::Itertools;

pub fn part_2_range(input: &str) -> i64 {
    let (seeds_str, categories_str) = input.split_once("\n\n").unwrap();
    let mut ranges = to_seed_ranges(seeds_str);
    let categories = to_categories(categories_str);

    ranges.sort();
    println!("{:?}", ranges);
    for category in categories {
        ranges = split_ranges(&ranges, &category.source_ranges());
        println!("{:?} (source)", ranges);
        ranges = category.map_to_destination_ranges(&ranges);
        println!("{:?} (mapped)\n", ranges)
    }

    ranges.iter()
        .flat_map(|range|
            once(range.0).chain(once(range.1))
        )
        .min().unwrap()
}

trait Range {
    fn contains_value(&self, value: i64) -> bool;
    fn contains_range(&self, range: (i64, i64)) -> bool;
}

impl Range for (i64, i64) {
    fn contains_value(&self, value: i64) -> bool {
        self.0 <= value && value < self.1
    }
    fn contains_range(&self, range: (i64, i64)) -> bool {
        self.contains_value(range.0) && self.contains_value(range.1)
    }
}

fn to_seed_ranges(input: &str) -> Vec<(i64, i64)> {
    let mut parts = input.split_once(':').unwrap().1.split_whitespace();
    let mut seeds = Vec::<(i64, i64)>::new();

    while let Some(start_str) = parts.next() {
        let end_str = parts.next().unwrap_or("");
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str.parse::<i64>().unwrap();
        seeds.push((start, start+end-1));
    }
    seeds.sort();
    seeds
}

trait Ranges {
    fn split(&mut self, ranges: &[(i64, i64)]);
}

impl Ranges for Vec<(i64, i64)> {
    fn split(&mut self, source_ranges: &[(i64, i64)]) {
        let mut i = 0;
        while i < self.len() {
            for source_range in source_ranges {
                if source_range.contains_range(self[i]) { break }
                else if source_range.contains_value(self[i].0) {
                    self.push((source_range.1+1, self[i].1));
                    self[i].1 = source_range.1;
                    break
                }
            }
            i += 1;
        }
    }
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

trait Category {
    fn map_to_destination(&self, source: i64) -> i64;
    fn map_to_destination_ranges(&self, source_range: &[(i64, i64)]) -> Vec<(i64, i64)>;
    fn source_ranges(&self) -> Vec<(i64, i64)>;
}

impl Category for Vec<Map> {
    fn map_to_destination(&self, source: i64) -> i64 {
        if let Some(map) = self.iter().rfind(|map| map.in_source_range(source)) {
            map.map_to_destination_range(source)
        } else {
            source
        }
    }
    fn map_to_destination_ranges(&self, source_range: &[(i64, i64)]) -> Vec<(i64, i64)> {
        println!("Mapping using ranges {:?}", self.source_ranges());
        source_range.iter().map(|range| {
            let destination_range_start = self.map_to_destination(range.0);
            if destination_range_start == 0 {
                println!("Mapped {} to 0", range.0);
            }
            let destination_range_end = self.map_to_destination(range.1);
            (destination_range_start, destination_range_end)
        }).collect()
    }
    fn source_ranges(&self) -> Vec<(i64, i64)> {
        self.iter().map(|map| (map.source_range_start, map.source_range_start + map.range_length - 1)).sorted().collect()
    }
}

fn to_categories(categories_str: &str) -> Vec<Vec<Map>> {
    categories_str.split("\n\n").map(to_maps).collect()
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
fn sample_input_part_3() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2_range(input), 46)
}

struct SeedRules {
    source_start: i64,
    source_end: i64,
    dist_between_dest_and_source: i64,
}

struct SeedRange {
    start: i64,
    end: i64,
}

fn is_within_range(val: i64, rule: &SeedRules) -> bool {
    val >= rule.source_start && val < rule.source_end
}

fn change_seed_ranges(seed_ranges: &mut Vec<SeedRange>, rules: &[SeedRules]) {
    let mut i = 0;
    while i < seed_ranges.len() {
        for rule in rules {
            if is_within_range(seed_ranges[i].start, rule)
                && is_within_range(seed_ranges[i].end, rule)
            {
                seed_ranges[i].start += rule.dist_between_dest_and_source;
                seed_ranges[i].end += rule.dist_between_dest_and_source;
                break;
            } else if is_within_range(seed_ranges[i].start, rule) {
                seed_ranges.push(SeedRange {
                    start: rule.source_end,
                    end: seed_ranges[i].end
                });
                seed_ranges[i].start += rule.dist_between_dest_and_source;
                seed_ranges[i].end = rule.source_end + rule.dist_between_dest_and_source;
                break;
            }
        }
        i += 1;
    }
}

fn split_ranges(ranges: &[(i64, i64)], source_ranges: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut splitted = false;
    println!("Splitting {:?} with {:?}", ranges, source_ranges);
    let (mut starts, mut ends): (HashSet<i64>, HashSet<i64>) = ranges.iter().map(|(a, b)| (a, b)).unzip();

    for source_range in source_ranges {
        if ranges.iter().any(|range| range.contains_value(source_range.0) && source_range.0 != range.0 ) {
            println!("{} is in {:?}", source_range.0, ranges.iter().find(|range| range.contains_value(source_range.0)).unwrap());
            if !starts.contains(&source_range.0) {
                ends.insert(source_range.0);
                starts.insert(source_range.0+1);
            }

            splitted = true;
        }
        if ranges.iter().any(|range| range.contains_value(source_range.1) && source_range.1 != range.1 ) {
            println!("{} is in {:?}", source_range.1, ranges.iter().find(|range| range.contains_value(source_range.1)).unwrap());
            ends.insert(source_range.1);
            starts.insert(source_range.1+1);
            splitted = true;
        }
    }
    if !splitted {
        println!("No splitting");
    }
    let mut starts_vec = Vec::from_iter(starts);
    starts_vec.sort();
    let mut ends_vec = Vec::from_iter(ends);
    ends_vec.sort();
    zip(starts_vec, ends_vec).collect()
}

#[test]
fn split_ranges_when_no_neighboring_ranges() {
    let ranges = vec![(57, 69), (81, 94)];
    let source_ranges = vec![(0, 6), (7, 10), (11, 52), (53, 60)];
    let splitted = split_ranges(&ranges, &source_ranges);
    assert_eq!(splitted, vec![(57, 60), (61, 69), (81, 94)])
}

#[test]
fn split_ranges_when_neighboring_ranges() {
    let ranges = vec![(46, 49), (54, 62), (74, 87)];
    let source_ranges = vec![(45, 63), (64, 76), (77, 99)];
    let splitted = split_ranges(&ranges, &source_ranges);
    assert_eq!(splitted, vec![(46, 49), (54, 62), (74, 76), (77, 87)])
}

#[test]
fn split_ranges_when_neighboring_ranges_and_only_start_is_contained() {
    let ranges = vec![(46, 49), (54, 62), (74, 87)];
    let source_ranges = vec![(47, 63), (64, 67), (88, 99)];
    let splitted = split_ranges(&ranges, &source_ranges);
    assert_eq!(splitted, vec![(46, 47), (48, 49), (54, 62), (74, 87)])
}

#[test]
fn split_ranges_when_neighboring_ranges_and_only_end_is_contained() {
    let mut ranges = vec![(46, 49), (54, 62), (74, 88)];
    let source_ranges = vec![(45, 63), (64, 67), (73, 86)];
    ranges.split(&source_ranges);
    assert_eq!(ranges, vec![(46, 49), (54, 62), (74, 86), (87, 88)])
}

#[test]
fn task_input_part_4() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_2_range(input), 84206669)
}