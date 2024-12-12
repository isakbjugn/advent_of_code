use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let garden = Grid::from_str(input).expect("Could not parse input into garden");
    let groups = find_contiguous_groups(&garden);

    groups.into_iter()
        .map(|group| group.len() * find_faces(group, &garden).len())
        .sum()
}

fn find_contiguous_groups(garden: &Grid) -> Vec<HashSet<Position>> {
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
    groups
}

fn find_faces(group: HashSet<Position>, garden: &Grid) -> HashSet<(Position, Direction)> {
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
    facing_out
}

fn calculate_sides(group: HashSet<Position>, garden: &Grid) -> usize {
    let faces = find_faces(group.clone(), garden);
    // println!("{} positions in group: {:?}", group.len(), group);
    // println!("{} faces in group: {:?}", faces.len(), faces);

    let mut sides: Vec<HashSet<(Position, Direction)>> = vec![];
    for face in faces.iter() {
        if sides.iter().any(|set| set.iter().contains(face)) {
            continue
        }
        let side_direction = face.1;
        let mut side: HashSet<(Position, Direction)> = HashSet::from([*face]);
        let mut queue: HashSet<(Position, Direction)> = faces.iter().filter(|(_, direction)| direction == &side_direction).cloned().collect();

        'side: loop {
            for (side_position, _) in side.iter() {
                if let Some(&neighbor) = queue.iter().find(|(position, _)| garden.are_neighbors(side_position, position)) {
                    queue.remove(&neighbor);
                    side.insert(neighbor);
                    continue 'side;
                }
            }
            break;
        }
        sides.push(side);
    }
    // println!("{} sides in group: {:?}\n", sides.len(), sides);
    sides.len()
}

pub fn part_2(input: &str) -> usize {
    let garden = Grid::from_str(input).expect("Could not parse input into garden");
    let groups = find_contiguous_groups(&garden);

    groups.into_iter()
        .map(|group| group.len() * calculate_sides(group, &garden))
        .sum()
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
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_12_1.txt");
    assert_eq!(part_2(input), 80)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_12_4.txt");
    assert_eq!(part_2(input), 236)
}

#[test]
fn sample_input_part_2_3() {
    let input = include_str!("../input/sample_12_5.txt");
    assert_eq!(part_2(input), 368)
}

#[test]
fn sample_input_part_2_4() {
    let input = include_str!("../input/sample_12_3.txt");
    assert_eq!(part_2(input), 1206)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_1(input), 1465112)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_2(input), 893790)
}
