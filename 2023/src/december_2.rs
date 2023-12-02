struct Cubes {
    red: i8,
    green: i8,
    blue: i8
}

const RED_LIMIT: i8 = 12;
const GREEN_LIMIT: i8 = 13;
const BLUE_LIMIT: i8 = 14;

impl Cubes {
    fn is_possible(&self) -> bool {
        self.red <= RED_LIMIT && self.green <= GREEN_LIMIT && self.blue <= BLUE_LIMIT
    }
    fn power(&self) -> i32 {
        self.red as i32 * self.green as i32 * self.blue as i32
    }
}

pub fn part_1(input: &str) -> i16 {

    input.lines().map(to_game_and_cubes)
        .map(|(game, cubes_str)| (game, to_max_cubes(cubes_str)))
        .filter(|(_, cubes)| cubes.is_possible())
        .map(|(game, _)| game as i16)
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input.lines().map(to_game_and_cubes)
        .map(|(_, cubes_str)| to_max_cubes(cubes_str))
        .map(|cubes| cubes.power())
        .sum()
}

fn to_game_and_cubes(line: &str) -> (i8, &str) {
    let (game_str, cubes_str) = line.split_once(": ").unwrap();
    (game_str.get(5..).unwrap().parse::<i8>().unwrap(), cubes_str)
}

fn to_max_cubes(cubes_str: &str) -> Cubes {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for set in cubes_str.split("; ") {
        let colors = set.split(", ");
        for color in colors {
            let (count_str, color) = color.split_once(' ').unwrap();
            let count = count_str.parse::<i8>().unwrap();
            match color {
                "red" if count > max_red => max_red = count,
                "green" if count > max_green => max_green = count,
                "blue" if count > max_blue => max_blue = count,
                _ => ()
            }
        }
    }
    Cubes {
        red: max_red,
        green: max_green,
        blue: max_blue
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_1(input), 8)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_2(input), 2286)
}