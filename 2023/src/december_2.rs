pub fn part_1(input: &str) -> i16 {
    const RED_LIMIT: i8 = 12;
    const GREEN_LIMIT: i8 = 13;
    const BLUE_LIMIT: i8 = 14;

    input.lines().map(to_game_and_cubes)
        .map(|(game, cubes_str)| (game, to_max_cubes(cubes_str)))
        .filter(|(_, max_cubes)| max_cubes.0 <= RED_LIMIT && max_cubes.1 <= GREEN_LIMIT && max_cubes.2 <= BLUE_LIMIT)
        .map(|(game, _)| game as i16)
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input.lines().map(to_game_and_cubes)
        .map(|(_, cubes_str)| to_max_cubes(cubes_str))
        .map(|(red, green, blue)| red as i32 * green as i32 * blue as i32)
        .sum()
}

fn to_game_and_cubes(line: &str) -> (i8, &str) {
    let (game_str, cubes_str) = line.split_once(": ").unwrap();
    (game_str.get(5..).unwrap().parse::<i8>().unwrap(), cubes_str)
}

fn to_max_cubes(cubes_str: &str) -> (i8, i8, i8) {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for set in cubes_str.split("; ") {
        let colors = set.split(", ");
        for color in colors {
            let (count_str, color) = color.split_once(" ").unwrap();
            let count = count_str.parse::<i8>().unwrap();
            match color {
                "red" if count > max_red => max_red = count,
                "green" if count > max_green => max_green = count,
                "blue" if count > max_blue => max_blue = count,
                _ => ()
            }
        }
    }
    (max_red, max_green, max_blue)
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