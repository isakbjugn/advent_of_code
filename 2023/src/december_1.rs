pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)))
        .map(|chars| chars.clone().next().unwrap().to_string() + &chars.clone().last().unwrap().to_string())
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.to_two_digit_string().parse::<i32>().unwrap())
        .sum()
}

trait CalibrationString {
    fn to_two_digit_string(&self) -> String;
    fn index_of_first_number(&self) -> Option<(&str, usize)>;
    fn index_of_last_number(&self) -> Option<(&str, usize)>;
}

impl CalibrationString for &str {

    fn to_two_digit_string(&self) -> String {
        let mut working_string;
        working_string = match self.index_of_first_number() {
            Some(("one", index)) => replace_char_at_index(self, index, '1'),
            Some(("two", index)) => replace_char_at_index(self, index, '2'),
            Some(("three", index)) => replace_char_at_index(self, index, '3'),
            Some(("four", index)) => replace_char_at_index(self, index, '4'),
            Some(("five", index)) => replace_char_at_index(self, index, '5'),
            Some(("six", index)) => replace_char_at_index(self, index, '6'),
            Some(("seven", index)) => replace_char_at_index(self, index, '7'),
            Some(("eight", index)) => replace_char_at_index(self, index, '8'),
            Some(("nine", index)) => replace_char_at_index(self, index, '9'),
            _ => self.to_string()
        };
        working_string = match self.index_of_last_number() {
            Some(("one", index)) => replace_char_at_index(working_string.as_str(), index, '1'),
            Some(("two", index)) => replace_char_at_index(working_string.as_str(), index, '2'),
            Some(("three", index)) => replace_char_at_index(working_string.as_str(), index, '3'),
            Some(("four", index)) => replace_char_at_index(working_string.as_str(), index, '4'),
            Some(("five", index)) => replace_char_at_index(working_string.as_str(), index, '5'),
            Some(("six", index)) => replace_char_at_index(working_string.as_str(), index, '6'),
            Some(("seven", index)) => replace_char_at_index(working_string.as_str(), index, '7'),
            Some(("eight", index)) => replace_char_at_index(working_string.as_str(), index, '8'),
            Some(("nine", index)) => replace_char_at_index(working_string.as_str(), index, '9'),
            _ => working_string
        };

        let digits = working_string
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>();

        digits.first().unwrap().to_string() + &digits.last().unwrap().to_string()
    }

    fn index_of_first_number(&self) -> Option<(&str, usize)> {
        ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            .into_iter()
            .filter_map(|number| {
                self.find(number).map(|index| (number, index))
            })
            .min_by_key(|&(_, index)| index)
    }
    fn index_of_last_number(&self) -> Option<(&str, usize)> {
        ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            .into_iter()
            .filter_map(|number| {
                self.rfind(number).map(|index| (number, index))
            })
            .max_by_key(|&(_, index)| index)
    }
}

fn replace_char_at_index(original: &str, index: usize, new_char: char) -> String {
    let mut chars: Vec<char> = original.chars().collect();

    if let Some(char_to_replace) = chars.get_mut(index) {
        *char_to_replace = new_char;
    }

    chars.into_iter().collect()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_1_1.txt");
    assert_eq!(part_1(input), 142)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_1_2.txt");
    assert_eq!(part_2(input), 281)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_1_3.txt");
    assert_eq!(part_2(input), 205)
}

#[test]
fn sample_input_part_2_3() {
    let input = include_str!("../input/sample_1_4.txt");
    assert_eq!(part_2(input), 1326)
}

#[test]
fn sample_input_part_2_4() {
    let input = include_str!("../input/sample_1_5.txt");
    assert_eq!(part_2(input), 673)
}