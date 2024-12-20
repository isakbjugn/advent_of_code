use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str, cost_margin: usize) -> usize {
    let racetrack = Grid::from_str(input).expect("Could not create maze");
    let optimal_path = racetrack.a_star_path().expect("Should be a valid path");
    racetrack.best_cheats(optimal_path, cost_margin, 2)
}

pub fn part_2(input: &str, cost_margin: usize) -> usize {
    let racetrack = Grid::from_str(input).expect("Could not create maze");
    let optimal_path = racetrack.a_star_path().expect("Should be a valid path");
    racetrack.best_cheats(optimal_path, cost_margin, 20)
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
    path: Vec<Position>,
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
    fn is_goal_state_20(&self, vertex: &Vertex) -> bool {
        self.find('E').expect("Must be an end") == vertex.position
    }
    fn a_star_path(&self) -> Option<Vec<Position>> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = self.find('S').expect("Must be a start position");
        let end_pos = self.find('E').expect("Must be an end position");
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: end_pos.manhattan_distance(start_pos),
            vertex: Vertex {
                position: start_pos,
            },
            path: vec![start_pos]
        });

        while let Some(AStarState { cost, vertex, path, .. }) = heap.pop() {
            if self.is_goal_state_20(&vertex) {
                return Some(path)
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
                };

                if !distances.contains_key(&new_vertex) || new_cost < distances[&new_vertex] {
                    heap.push(AStarState {
                        cost: new_cost,
                        estimated_total_cost,
                        vertex: new_vertex,
                        path: path.iter().cloned().chain([next_position]).collect()
                    });
                }
            }
        }

        None
    }
    fn best_cheats(&self, optimal_path: Vec<Position>, cost_margin: usize, cheat_time: usize) -> usize {
        optimal_path.iter().enumerate()
            .map(|(cheat_start_index, cheat_start)| {
                optimal_path.iter().enumerate().skip(cheat_start_index + cost_margin)
                    .filter(|(cheat_end_index, cheat_end)| {
                        let distance = cheat_start.manhattan_distance(**cheat_end);
                        if distance <= cheat_time && cheat_start_index + distance <= cheat_end_index - cost_margin {
                            return true
                        }
                        false
                    })
                    .count()
            })
            .sum()
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_20.txt");
    assert_eq!(part_1(input, 1), 44)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_20.txt");
    assert_eq!(part_2(input, 50), 285)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_20.txt");
    assert_eq!(part_1(input, 100), 1355)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_20.txt");
    assert_eq!(part_2(input, 100), 1007335)
}
