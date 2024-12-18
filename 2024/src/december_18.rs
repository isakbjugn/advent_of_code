use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str, width: usize, height: usize, bytes: usize) -> usize {
    let mut memory = Grid::new(width, height, '.');
    input.lines()
        .take(bytes)
        .map(|line| line.split_once(',').expect("Falling bytes have commas"))
        .map(|(x, y)| Position { x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() } )
        .for_each(|position| memory.set(position, '#'));
    
    memory.a_star_search_18().expect("Should be a shortest path")
}

pub fn part_2(input: &str, width: usize, height: usize, start_bytes: usize) -> String {
    let mut memory = Grid::new(width, height, '.');
    let bytes: Vec<Position> = input.lines()
        .map(|line| line.split_once(',').expect("Falling bytes have commas"))
        .map(|(x, y)| Position { x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() } )
        .collect();
    bytes.iter().take(start_bytes)
        .for_each(|position| memory.set(*position, '#'));
    
    for byte in bytes.into_iter().skip(start_bytes) {
        memory.set(byte, '#');
        match memory.a_star_search_18() {
            Some(_) => continue,
            None => return format!("{},{}", byte.x, byte.y)
        }
    }
    String::new()
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Vertex {
    position: Position,
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
    fn is_goal_state_18(&self, vertex: &Vertex) -> bool {
        vertex.position == Position { x: self.width() - 1, y: self.height() - 1 }
    }
    fn a_star_search_18(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = Position { x: 0, y: 0 };
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: self.manhattan_distance_18(start_pos),
            vertex: Vertex {
                position: start_pos,
            }
        });

        while let Some(AStarState { cost, vertex, .. }) = heap.pop() {
            if self.is_goal_state_18(&vertex) {
                return Some(cost)
            }

            if let Some(&d) = distances.get(&vertex) {
                if cost > d {
                    continue;
                }
            }

            distances.insert(vertex.clone(), cost);

            for next_position in self.neighbor_iter(&vertex.position) {
                if self.get(&next_position) == Some('#') { continue }
                let new_cost = cost + 1;
                let estimated_total_cost = new_cost + self.manhattan_distance_18(next_position);
                let new_vertex = Vertex {
                    position: next_position,
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
    fn manhattan_distance_18(&self, pos: Position) -> usize {
        let dx = self.width() - 1 - pos.x;
        let dy = self.height() - 1 - pos.y;
        dx + dy
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_18.txt");
    assert_eq!(part_1(input, 7, 7, 12), 22)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_18.txt");
    assert_eq!(part_2(input, 7, 7, 12), "6,1")
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_18.txt");
    assert_eq!(part_1(input, 71, 71, 1024), 250)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_18.txt");
    assert_eq!(part_2(input, 71, 71, 1024), "56,8")
}
