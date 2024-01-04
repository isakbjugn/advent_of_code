use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let grid = Grid::new(input).unwrap();
    let start_time = Instant::now();
    let result = grid.dijkstra().unwrap();
    println!("Dijkstra took {} ms", start_time.elapsed().as_millis());
    result
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Vertex {
    position: Position,
    direction: Option<Direction>,
    straight_path: usize,
}

// Define a wrapper type for the heap
#[derive(Clone, Eq, PartialEq)]
struct DijkstraState {
    cost: usize,
    vertex: Vertex,
}

// Implement `Ord` for `State`. We want to order states by cost in reverse order since `BinaryHeap` is a max heap.
impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    fn legal_directions(&self, vertex: &Vertex) -> Vec<Direction> {
        if let Some(direction) = vertex.direction {
            let possible_directions: HashSet<Direction> = HashSet::from_iter(self.possible_directions(vertex.position));
            let perpendicular = direction.perpendicular();

            let legal_directions = match vertex.straight_path {
                n if n < 3 => HashSet::from([perpendicular.0, perpendicular.1, direction]),
                _ => HashSet::from([perpendicular.0, perpendicular.1]),
            };

            legal_directions.intersection(&possible_directions).cloned().collect()
        } else {
            vec!(Direction::South, Direction::East)
        }
    }
    fn is_goal_state(&self, position: Position) -> bool {
        position.x == self.width - 1 && position.y == self.height - 1
    }
    fn dijkstra(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances: HashMap<Vertex, usize> = HashMap::new();

        let start_pos = Position { x: 0, y: 0 };
        heap.push(DijkstraState {
            cost: 0,
            vertex: Vertex {
                position: start_pos,
                direction: None,
                straight_path: 0
            }
        });

        while let Some(DijkstraState { cost, vertex }) = heap.pop() {
            if self.is_goal_state(vertex.position) {
                return Some(cost);
            }

            if let Some(&d) = distances.get(&vertex) {
                if cost > d {
                    continue;
                }
            }

            distances.insert(vertex.clone(), cost);

            for direction in self.legal_directions(&vertex) {
                if let Some(neighbor) = self.neighbor_in_direction_from_position(vertex.position, direction) {
                    let new_cost = cost + self.data[neighbor.y][neighbor.x].to_digit(10).unwrap() as usize;
                    let new_vertex = Vertex {
                        position: neighbor,
                        direction: Some(direction),
                        straight_path: count_straight_path(vertex.straight_path, vertex.direction, direction)
                    };
                    heap.push(DijkstraState {
                        cost: new_cost,
                        vertex: new_vertex
                    });
                }
            }
        }

        None // No path found
    }
    fn a_star_search(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = Position { x: 0, y: 0 };
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: self.manhattan_distance(start_pos),
            vertex: Vertex {
                position: start_pos,
                direction: None,
                straight_path: 0
            }
        });

        while let Some(AStarState { cost, estimated_total_cost, vertex }) = heap.pop() {
            if self.is_goal_state(vertex.position) {
                return Some(cost);
            }

            if let Some(&d) = distances.get(&vertex) {
                if cost > d {
                    continue;
                }
            }

            distances.insert(vertex.clone(), cost);

            for direction in self.legal_directions(&vertex) {
                if let Some(neighbor) = self.neighbor_in_direction_from_position(vertex.position, direction) {
                    let new_cost = cost + self.data[neighbor.y][neighbor.x].to_digit(10).unwrap() as usize;
                    let estimated_total_cost = new_cost + self.manhattan_distance(neighbor);
                    let new_vertex = Vertex {
                        position: neighbor,
                        direction: Some(direction),
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
        }

        None // No path found
    }
    fn manhattan_distance(&self, pos: Position) -> usize {
        let dx = self.width - 1 - pos.x;
        let dy = self.height - 1 - pos.y;
        dx + dy
    }
}

fn count_straight_path(straight_path: usize, previous_direction: Option<Direction>, new_direction: Direction) -> usize {
    match previous_direction {
        Some(direction) if direction == new_direction => straight_path + 1,
        _ => 1,
    }
}

pub fn part_2(input: &str) -> u32 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_17.txt");
    assert_eq!(part_1(input), 102)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_17.txt");
    assert_eq!(part_2(input), 0)
}