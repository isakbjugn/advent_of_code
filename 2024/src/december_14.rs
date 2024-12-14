use regex::Regex;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str, floor_width: u32, floor_height: u32) -> u32 {
    let (first, second, third, fourth) = input.lines()
        .map(|line| to_robot(line, floor_width, floor_height).expect("Could not create robot from str"))
        .map(|robot| robot.proceed(100))
        .map(|robot| robot.quadrant())
        .fold((0, 0, 0, 0), |acc, (a, b, c, d)| {
            (acc.0 + a, acc.1 + b, acc.2 + c, acc.3 + d)
        });
    first * second * third * fourth
}

fn to_robot(robot_str: &str, floor_width: u32, floor_height: u32) -> Option<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    if let Some(captures) = re.captures(robot_str) {
        let x = captures.get(1)?.as_str().parse::<usize>().ok()?;
        let y = captures.get(2)?.as_str().parse::<usize>().ok()?;
        let vel_x = captures.get(3)?.as_str().parse::<isize>().ok()?;
        let vel_y = captures.get(4)?.as_str().parse::<isize>().ok()?;
        Some(Robot { position: Position { x, y }, velocity: Velocity { x: vel_x, y: vel_y }, width: floor_width, height: floor_height })
    } else {
        None
    }
}

struct Velocity {
    x: isize,
    y: isize,
}

struct Robot {
    position: Position,
    velocity: Velocity,
    width: u32,
    height: u32,
}

impl Robot {
    pub fn proceed(self, seconds: u32) -> Self {
        let x = (self.position.x as isize + self.velocity.x * seconds as isize) % self.width as isize;
        let x = if x < 0 { x + self.width as isize } else { x };
        let y = (self.position.y as isize + self.velocity.y * seconds as isize) % self.height as isize;
        let y = if y < 0 { y + self.height as isize } else { y };

        Self { position: Position { x: x as usize, y: y as usize }, velocity: self.velocity, width: self.width, height: self.height }
    }
    pub fn quadrant(&self) -> (u32, u32, u32, u32) {
        let Position { x, y } = self.position;
        let center_x = (self.width as usize - 1) / 2;
        let center_y = (self.height as usize - 1) / 2;
        
        match (x < center_x, x > center_x, y < center_y, y > center_y) {
            (false, true, true, false) => (1, 0, 0, 0),
            (false, true, false, true) => (0, 1, 0, 0),
            (true, false, false, true) => (0, 0, 1, 0),
            (true, false, true, false) => (0, 0, 0, 1),
            _ => (0, 0, 0, 0)
        }
    }
}

pub fn part_2(input: &str, floor_width: u32, floor_height: u32) -> u32 {
    let mut robots: Vec<Robot> = input.lines()
        .map(|line| to_robot(line, floor_width, floor_height).expect("Could not create robot from str"))
        .collect();
    
    let mut seconds = 0;
    loop {
        let mut grid = Grid::new(floor_height as usize, floor_width as usize, '.');
        for robot in robots.iter() {
            grid.set(robot.position, '1');
        }
        if has_eight_consecutive(&grid) {
            println!("Seconds: {}", seconds);
            println!("{}", grid);
            return seconds
        }
        robots = robots.into_iter().map(|robot| robot.proceed(1)).collect();
        seconds += 1;
    }
}

fn has_eight_consecutive(grid: &Grid) -> bool {
    for row in 0..grid.height() {
        let mut count = 0;
        for col in 0..grid.width() {
            if grid.get(&Position { x: col, y: row }) != Some('.') {
                count += 1;
                if count == 8 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }
    false
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_1(input, 11, 7), 12)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_14.txt");
    assert_eq!(part_1(input, 101, 103), 229839456)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_14.txt");
    assert_eq!(part_2(input, 101, 103), 7138)
}
