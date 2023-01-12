use std::collections::{
    HashMap,
    HashSet,
    VecDeque
};

struct Queue<T> {
    pub items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push_back(v)
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from empty queue.")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Square {
    letter: char,
    height: u32,
    position: (u32, u32)
}

impl Square {
    pub fn new(height: char, row: u32, col: u32) -> Square {
        Square {
            letter: height,
            height: match height {
                'S' => 0,
                'E' => 25,
                x => x as u32 - 97
            },
            position: (row, col)
        }
    }
}

type Vertex = Vec<Square>;

type Graph = HashMap<Square, Vertex>;

fn init_height_map(input: &str) -> Vec<Vec<Square>> {
    let mut height_map = Vec::<Vec<Square>>::new();
    for (row, line) in input.lines().enumerate() {
        let mut new_row = Vec::new();
        for (col, height) in line.chars().enumerate() {
            new_row.push(Square::new(height, row as u32, col as u32));
        }
        height_map.push(new_row);
    }
    height_map
}

fn init_adjacency_graph(height_map: &Vec<Vec<Square>>) -> Graph {
    let height = height_map.len();
    let width = height_map[0].len();

    let mut graph: Graph = HashMap::new();
    for (row, line) in height_map.iter().enumerate() {
        for (col, square) in line.iter().enumerate() {
            let mut vertices = Vec::new();
            if row > 0 && height_map[row - 1][col].height <= square.height + 1 {
                vertices.push(height_map[row - 1][col])
            }
            if row < height - 1 && height_map[row + 1][col].height <= square.height + 1 {
                vertices.push(height_map[row + 1][col])
            }
            if col > 0 && height_map[row][col - 1].height <= square.height + 1 {
                vertices.push(height_map[row][col - 1])
            }
            if col < width - 1 && height_map[row][col + 1].height <= square.height + 1 {
                vertices.push(height_map[row][col + 1])
            }
            graph.insert(*square, vertices);
        }
    }
    graph
}
fn bfs(graph: &Graph, start_node: Square, end_node: Square) -> Option<Vec<Option<Square>>> {
    let mut queue = Queue::new();
    queue.enqueue(start_node);

    let mut visited_vertices = HashSet::new();
    visited_vertices.insert(start_node);

    let mut prev = HashMap::<Square, Option<Square>>::new();
    prev.insert(start_node, None);

    'outer: while !queue.is_empty() {
        let current_node = queue.dequeue();
        for v in graph.get(&current_node).unwrap().iter() {
            if *v == end_node {
                prev.insert(*v, Some(current_node));
                break 'outer;
            }

            if !visited_vertices.contains(v) {
                queue.enqueue(*v);
                visited_vertices.insert(*v);
                prev.insert(*v, Some(current_node));
            }
        }
    }

    if !prev.contains_key(&end_node) {
        return None;
    }

    let mut path = Vec::new();
    let mut at = Some(end_node);
    while at != None {
        path.push(at);
        at = *prev.get(&at.unwrap_or(start_node)).unwrap();
    }

    path.reverse();

    match path[0] {
        Some(x) if x == start_node => Some(path),
        _ => None,
    }
}

fn find_square(height_map: &Vec<Vec<Square>>, letter: char) -> Option<Square> {
    for row in height_map {
        for square in row {
            if square.letter == letter {
                return Some(*square)
            }
        }
    }
    None
}

pub fn part_1(input: &str) -> u32 {
    let height_map = init_height_map(input);
    let adj_graph = init_adjacency_graph(&height_map);
    let start_node = find_square(&height_map, 'S').unwrap();
    let end_node = find_square(&height_map, 'E').unwrap();

    find_shortest_route(&adj_graph, start_node, end_node).unwrap()
}

pub fn part_2(input: &str) -> u32 {
    let height_map = init_height_map(input);
    let adj_graph = init_adjacency_graph(&height_map);
    let end_node = find_square(&height_map, 'E').unwrap();
    let mut shortest_route = u32::MAX;

    for line in height_map {
        for square in line {
            if square.height == 0 {
                if let Some(route) = find_shortest_route(&adj_graph, square, end_node) {
                    if route < shortest_route {
                        shortest_route = route;
                    }
                }
            }
        }
    }
    shortest_route
}

fn find_shortest_route(graph: &Graph, start_node: Square, end_node: Square) -> Option<u32> {
    let path = bfs(graph, start_node, end_node);
    path.map(|s| s.len() as u32 - 1)
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_1(input), 31)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_2(input), 29)
}
