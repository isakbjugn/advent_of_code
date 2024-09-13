use std::iter::{once};
use itertools::Itertools;

pub fn part_2_range(input: &str) -> i64 {
    let (seeds_str, categories_str) = input.split_once("\n\n").unwrap();
    let mut ranges = to_seed_ranges(seeds_str);
    if let Some(overlap) = ranges.find_overlapping() {
        panic!("Ranges overlap: {:?}", overlap)
    }
    let categories = to_categories(categories_str);

    ranges.sort();
    println!("{:?}", ranges);
    for category in categories {
        ranges.split(&category.source_ranges());
        if let Some(overlap) = ranges.find_overlapping() {
            panic!("Ranges overlap: {:?}", overlap)
        }
        println!("{:?} (source)", ranges);
        ranges = category.map_to_destination_ranges(&ranges);
        println!("{:?} (mapped)\n", ranges);
        if let Some(overlap) = ranges.find_overlapping() {
            panic!("Ranges overlap: {:?}", overlap)
        }
    }
    if let Some(overlap) = ranges.find_overlapping() {
        panic!("Ranges overlap: {:?}", overlap)
    }
    println!("{:?} (source)", ranges);

    ranges.iter()
        .flat_map(|range|
            once(range.0).chain(once(range.1))
        )
        .min().unwrap()
}

trait Range {
    fn contains(&self, value: i64) -> bool;
    fn contains_range(&self, range: (i64, i64)) -> bool;
}

impl Range for (i64, i64) {
    fn contains(&self, value: i64) -> bool {
        self.0 <= value && value <= self.1
    }
    fn contains_range(&self, range: (i64, i64)) -> bool {
        self.contains(range.0) && self.contains(range.1)
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
    seeds.sort_by_key(|seed| seed.0);
    seeds
}

trait Ranges {
    fn split(&mut self, ranges: &[(i64, i64)]);
    #[allow(unused)]
    fn any_overlapping(&self) -> bool;
    fn find_overlapping(&self) -> Option<(i64, i64)>;
}

impl Ranges for Vec<(i64, i64)> {
    fn split(&mut self, source_ranges: &[(i64, i64)]) {
        let mut i = 0;
        while i < self.len() {
            for source_range in source_ranges {
                if source_range.contains_range(self[i]) { break }
                else if source_range.contains(self[i].0) {
                    let shortened_range = (self[i].0, source_range.1);
                    let new_range = (source_range.1+1, self[i].1);
                    println!("Splitting {:?} using {:?} into {:?} and {:?}", self[i], source_range, shortened_range, new_range);
                    self.push(new_range);
                    self[i] = shortened_range;
                    break
                }
                else if source_range.contains(self[i].1) {
                    let shortened_range = (self[i].0, source_range.0-1);
                    let new_range = (source_range.0, self[i].1);
                    println!("Splitting {:?} using {:?} into {:?} and {:?}", self[i], source_range, shortened_range, new_range);
                    self.push(new_range);
                    self[i] = shortened_range;
                    break
                }
            }
            i += 1;
        }
        self.sort();
    }
    fn any_overlapping(&self) -> bool {
        self.iter().tuple_windows().any(|(a, b)| a.1 >= b.0)
    }
    fn find_overlapping(&self) -> Option<(i64, i64)> {
        self.iter().tuple_windows().find(|(a, b)| a.1 >= b.0).map(|(a, b)| (a.1, b.0))
    }
}

#[test]
fn split_ranges_when_neighboring_ranges_and_only_start_is_contained() {
    let mut ranges = vec![(46, 49), (54, 62), (74, 87)];
    let source_ranges = vec![(47, 63), (64, 67), (88, 99)];
    ranges.split(&source_ranges);
    assert_eq!(ranges, vec![(46, 46), (47, 49), (54, 62), (74, 87)])
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
    fn map_to_destination_ranges(&self, source_range: &[(i64, i64)]) -> Vec<(i64, i64)> {
        println!("Mapping using ranges {:?}", self.source_ranges());
        source_range.iter().map(|range| {
            if self.any_overlap(*range) {
                panic!("Source range should already be splitted and thus contained, {:?}", range)
            }
            let destination_range_start = self.map_to_destination(range.0);
            if destination_range_start == 0 {
                println!("Mapped {} to 0", range.0);
            }
            let destination_range_end = self.map_to_destination(range.1);
            (destination_range_start, destination_range_end)
        })
            .sorted()
            .collect()
    }
    fn source_ranges(&self) -> Vec<(i64, i64)>;
    fn any_overlap(&self, range: (i64, i64)) -> bool {
        self.source_ranges().iter()
            .any(|source| source.contains(range.0) && !source.contains(range.1))
        || self.source_ranges().iter()
            .any(|source| !source.contains(range.0) && source.contains(range.1))
    }
}

impl Category for Vec<Map> {
    fn map_to_destination(&self, source: i64) -> i64 {
        if let Some(map) = self.iter().rfind(|map| map.in_source_range(source)) {
            map.map_to_destination_range(source)
        } else {
            source
        }
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

#[test]
fn split_ranges_when_no_neighboring_ranges() {
    let mut ranges = vec![(57, 69), (81, 94)];
    let source_ranges = vec![(0, 6), (7, 10), (11, 52), (53, 60)];
    ranges.split(&source_ranges);
    assert_eq!(ranges, vec![(57, 60), (61, 69), (81, 94)])
}

#[test]
fn split_ranges_when_neighboring_ranges() {
    let mut ranges = vec![(46, 49), (54, 62), (74, 87)];
    let source_ranges = vec![(45, 63), (64, 76), (77, 99)];
    ranges.split(&source_ranges);
    assert_eq!(ranges, vec![(46, 49), (54, 62), (74, 76), (77, 87)])
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

