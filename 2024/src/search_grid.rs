use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::direction::Direction;
use crate::position::Position;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Vertex {
    position: Position,
    direction: Option<Direction>,
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

trait SearchGrid {
    fn next_vertices(&self, vertex: Vertex, direction: Option<Direction>) -> Vec<(Vertex, Option<Direction>, usize)>;
    fn get_start_position(&self) -> Position;
    fn get_goal_position(&self) -> Position;
    fn is_goal_state(&self, vertex: &Vertex) -> bool;
    fn a_star_search(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = self.get_start_position();
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: self.get_goal_position().manhattan_distance(start_pos),
            vertex: Vertex {
                position: start_pos,
                direction: None,
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

            for (next_position, _, added_cost) in self.next_vertices(vertex.position, None) {
                let new_cost = cost + added_cost;
                let estimated_total_cost = new_cost + self.get_goal_position().manhattan_distance(next_position.position);
                let new_vertex = Vertex {
                    position: next_position.position,
                    direction: None,
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
    fn a_star_search_penalise_direction(&self, start_direction: Option<Direction>, turn_cost: usize) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        let start_pos = self.get_start_position();
        heap.push(AStarState {
            cost: 0,
            estimated_total_cost: self.get_goal_position().manhattan_distance(start_pos),
            vertex: Vertex {
                position: start_pos,
                direction: start_direction,
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
}