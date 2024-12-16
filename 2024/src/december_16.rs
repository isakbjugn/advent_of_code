use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use itertools::Itertools;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let maze = Grid::from_str(input).expect("Could not create maze");
    maze.a_star_search().expect("Should be a valid path")
}

pub fn part_2(input: &str) -> usize {
    0
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Vertex {
    position: Position,
    direction: Direction,
    straight_path: usize,
}

// Define a wrapper type for the heap
#[derive(Clone, Eq, PartialEq)]
struct AStarState {
    cost: usize,
    estimated_total_cost: usize,
    vertex: Vertex,
}

// Implement `Ord` for `State`. We want to order states by cost in reverse order since `BinaryHeap` is a max heap.
impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_total_cost.cmp(&self.estimated_total_cost)
    }
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn next_vertices(&self, position: Position, direction: Direction) -> Vec<(Position, Direction, usize)> {
        match self.next_value(&position, direction) {
            Some('#') => vec![(position, direction.clockwise(), 1000), (position, direction.counter_clockwise(), 1000)],
            Some(_) => vec![(self.next_position(&position, direction).unwrap(), direction, 1), (position, direction.clockwise(), 1000), (position, direction.counter_clockwise(), 1000)],
            _ => unreachable!("Should never check past walls")
        }
    }
    fn is_goal_state(&self, vertex: &Vertex) -> bool {
        self.find('E').expect("Must be an end") == vertex.position
    }
    fn a_star_search(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = self.find('S').expect("Must be a start position");
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: self.manhattan_distance(start_pos),
            vertex: Vertex {
                position: start_pos,
                direction: Direction::East,
                straight_path: 0
            }
        });


        while let Some(AStarState { cost, vertex, .. }) = heap.pop() {
            if self.is_goal_state(&vertex) {
                return Some(cost)
            }

            if let Some(&d) = distances.get(&vertex) {
                if cost > d {
                    continue;
                }
            }

            distances.insert(vertex.clone(), cost);
            
            for (next_position, next_direction, added_cost) in self.next_vertices(vertex.position, vertex.direction) {
                let new_cost = cost + added_cost;
                let estimated_total_cost = new_cost + self.manhattan_distance(next_position);
                let new_vertex = Vertex {
                    position: next_position,
                    direction: next_direction,
                    straight_path: count_straight_path(vertex.straight_path, vertex.direction, next_direction)
                };

                if !distances.contains_key(&new_vertex) || new_cost < distances[&new_vertex] {
                    heap.push(AStarState {
                        cost: new_cost,
                        estimated_total_cost,
                        vertex: new_vertex
                    });
                }
            }
        }

        None
    }
    fn manhattan_distance(&self, pos: Position) -> usize {
        let dx = self.width() - 1 - pos.x;
        let dy = self.height() - 1 - pos.y;
        dx + dy
    }
}

fn count_straight_path(straight_path: usize, previous_direction: Direction, new_direction: Direction) -> usize {
    match previous_direction == new_direction {
        true => straight_path + 1,
        false => 1,
    }
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_16_1.txt");
    assert_eq!(part_1(input), 7036)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_16_2.txt");
    assert_eq!(part_1(input), 11048)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_16_1.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_16_2.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_16.txt");
    assert_eq!(part_1(input), 115500)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_16.txt");
    assert_eq!(part_2(input), 0)
}
