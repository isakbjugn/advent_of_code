use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let maze = Grid::from_str(input).expect("Could not create maze");
    
    maze.a_star_search().expect("Should be a valid path")
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
    fn is_goal_state(&self, vertex: &Vertex) -> bool {
        self.find('E').expect("Must be an end") == vertex.position
    }
    fn a_star_search(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();
        let mut optimal_cost = None;

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
                if optimal_cost.is_none() {
                    optimal_cost = Some(cost)
                }
                continue;
            }

            if let Some(&d) = distances.get(&vertex) {
                if cost > d {
                    continue;
                }
            }

            distances.insert(vertex.clone(), cost);
            
            // Three possible choices:
            // 1. continue in current direction,
            let direction = vertex.direction;
            if let Some(neighbor) = self.next_position(&vertex.position, direction) {
                if self.get(&neighbor) != Some('#') {
                    let new_cost = cost + 1;
                    let estimated_total_cost = new_cost + self.manhattan_distance(neighbor);
                    let new_vertex = Vertex {
                        position: neighbor,
                        direction: direction,
                        straight_path: count_straight_path(vertex.straight_path, vertex.direction, direction)
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
            
            // 2. rotate clockwise,
            {
                let direction = vertex.direction.clockwise();
                let same_tile_but_rotated = vertex.position;
                
                let new_cost = cost + 1000;
                let estimated_total_cost = new_cost + self.manhattan_distance(same_tile_but_rotated);
                let new_vertex = Vertex {
                    position: same_tile_but_rotated,
                    direction: direction,
                    straight_path: count_straight_path(vertex.straight_path, vertex.direction, direction)
                };

                if !distances.contains_key(&new_vertex) || new_cost < distances[&new_vertex] {
                    heap.push(AStarState {
                        cost: new_cost,
                        estimated_total_cost,
                        vertex: new_vertex
                    });
                }
            }
            
            // 3. rotate counterclockwise
            {
                let direction = vertex.direction.counter_clockwise();
                let same_tile_but_rotated = vertex.position;

                let new_cost = cost + 1000;
                let estimated_total_cost = new_cost + self.manhattan_distance(same_tile_but_rotated);
                let new_vertex = Vertex {
                    position: same_tile_but_rotated,
                    direction: direction,
                    straight_path: count_straight_path(vertex.straight_path, vertex.direction, direction)
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

        optimal_cost // No path found
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

pub fn part_2(input: &str) -> u32 {
    0
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
    assert_eq!(part_2(input), 45)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_16_2.txt");
    assert_eq!(part_2(input), 64)
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
