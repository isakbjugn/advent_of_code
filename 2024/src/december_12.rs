use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let garden = Grid::from_str(input).expect("Could not parse input into garden");
    let mut groups: Vec<HashSet<Position>> = Vec::new();

    for (start_position, group_plant) in garden.iter() {
        if groups.iter().any(|set| set.iter().contains(&start_position)) {
            continue
        }
        let mut group: HashSet::<Position> = HashSet::new();
        let mut queue: VecDeque<Position> = VecDeque::from([start_position]);
        let mut visited: HashSet<Position> = HashSet::new();

        while let Some(position) = queue.pop_front() {
            match garden.get(&position) {
                Some(plant) if plant == group_plant => {
                    group.insert(position);
                    for neighbor in garden.neighbor_iter(&position) {
                        if !queue.contains(&neighbor) && !visited.contains(&neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
                Some(_) => (),
                None => unreachable!()
            }
            visited.insert(position);
        }
        groups.push(group);
    }
    
    groups.into_iter()
        .map(|group| group.len() * calculate_area(group, &garden))
        .sum()
}

fn calculate_area(group: HashSet<Position>, garden: &Grid) -> usize {
    let mut facing_out: HashSet<(Position, Direction)> = HashSet::new();
    let group_plant = garden
        .get(group.iter().find_or_first(|_| true).expect("Group should have at least one element"))
        .expect("First element should correspond to a plant");
    let all_directions = vec![Direction::North, Direction::East, Direction::South, Direction::West];
    
    for position in group {
        for &direction in all_directions.iter() {
            match garden.next_value(&position, direction) {
                Some(plant) if plant == group_plant => {
                    continue
                },
                Some(_) | None => {
                    facing_out.insert((position, direction));
                }
            }
        }
    }
    facing_out.len()
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_12_1.txt");
    assert_eq!(part_1(input), 140)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_12_2.txt");
    assert_eq!(part_1(input), 772)
}


#[test]
fn sample_input_part_1_3() {
    let input = include_str!("../input/sample_12_3.txt");
    assert_eq!(part_1(input), 1930)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_12_1.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_1(input), 1465112)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_2(input), 0)
}
