use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let racetrack = Grid::from_str(input).expect("Could not create maze");
    let optimal_time = racetrack.a_star_search_20().expect("Should be a valid path");
    // optimal_time
    racetrack.dijkstra_cheats_20(optimal_time).len()
}

pub fn part_2(input: &str) -> usize {
    0
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Vertex {
    position: Position,
    cheats: Option<(Position, Position)>
}

// Define a wrapper type for the heap
#[derive(Clone, Eq, PartialEq)]
struct AStarState {
    cost: usize,
    estimated_total_cost: usize,
    vertex: Vertex,
}

// Define a wrapper type for the heap
#[derive(Clone, Eq, PartialEq)]
struct DijkstraState {
    cost: usize,
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

impl Grid {
    fn is_goal_state_20(&self, vertex: &Vertex) -> bool {
        self.find('E').expect("Must be an end") == vertex.position
    }
    fn a_star_search_20(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = self.find('S').expect("Must be a start position");
        let end_pos = self.find('E').expect("Must be an end position");
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: end_pos.manhattan_distance(start_pos),
            vertex: Vertex {
                position: start_pos,
                cheats: None,
            }
        });

        while let Some(AStarState { cost, vertex, .. }) = heap.pop() {
            if self.is_goal_state_20(&vertex) {
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
                let estimated_total_cost = new_cost + end_pos.manhattan_distance(next_position);
                let new_vertex = Vertex {
                    position: next_position,
                    cheats: None,
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
    fn dijkstra_cheats_20(&self, optimal_cost: usize) -> HashSet<(Position, Position)> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();
        let mut best_cheats : HashSet<(Position, Position)> = HashSet::new();

        let start_pos = self.find('S').expect("Must be a start position");
        heap.push(DijkstraState {
            cost: 0,
            vertex: Vertex {
                position: start_pos,
                cheats: None,
            },
        });

        while let Some(DijkstraState { cost, vertex, .. }) = heap.pop() {
            if self.is_goal_state_20(&vertex) {
                println!("Reached goal state! cost: {}, cheats: {:?}", cost, vertex.cheats);
                // Add all positions in this path to the result set
                match vertex.cheats {
                    Some((start_cheat, end_cheat)) if cost <= optimal_cost - 100 => {
                        match best_cheats.insert((start_cheat, end_cheat)) {
                            true => println!("Added!"),
                            false => println!("Already inserted!")
                        }
                    }
                    _ => (),
                }
                continue;
            }

            if let Some(&d) = distances.get(&vertex) {
                if cost > d {
                    continue;
                }
            }

            distances.insert(vertex.clone(), cost);

            for next_position in self.neighbor_iter(&vertex.position) {
                let new_cheats = match (vertex.cheats, self.get(&next_position)) {
                    (None, Some('#')) => Some((vertex.position, next_position)),
                    (Some(cheats), _) if best_cheats.contains(&cheats) => continue,
                    (Some(_), Some('#')) => continue,
                    _ => vertex.cheats,
                };
                
                let new_cost = cost + 1;
                if new_cost > optimal_cost - 100 { continue }
                let new_vertex = Vertex {
                    position: next_position,
                    cheats: new_cheats,
                };

                if !distances.contains_key(&new_vertex) || new_cost < distances[&new_vertex] {
                    heap.push(DijkstraState {
                        cost: new_cost,
                        vertex: new_vertex,
                    });
                }
            }
        }

        best_cheats
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_20.txt");
    assert_eq!(part_1(input), 44)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_20.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_20.txt");
    assert_eq!(part_1(input), 1355)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_20.txt");
    assert_eq!(part_2(input), 0)
}
