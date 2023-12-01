
pub fn part_1(input: &str) -> i16 {
    input
        .lines()
        .map(|line| line.filter_digits())
        .map(|line| line.as_str().to_int_from_first_and_last())
        .sum()
}

pub fn part_2(input: &str) -> i16 {
    input
        .lines()
        .map(|line| line.replace_written_numbers_with_digits())
        .map(|line| line.as_str().filter_digits())
        .map(|line| line.as_str().to_int_from_first_and_last())
        .sum()
}

trait CalibrationString {
    fn filter_digits(&self) -> String;
    fn replace_written_numbers_with_digits(&self) -> String;
    fn value_and_index_of_first_number(&self) -> Option<(char, usize)>;
    fn value_and_index_of_last_number(&self) -> Option<(char, usize)>;
    fn to_int_from_first_and_last(&self) -> i16;
}

impl CalibrationString for &str {
    fn filter_digits(&self) -> String {
        self.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
    }
    fn replace_written_numbers_with_digits(&self) -> String {
        let mut working_string = self.to_string();
        if let Some((number, index)) = self.value_and_index_of_first_number() {
            working_string = replace_char_at_index(self, index, number);
        }

        if let Some((number, index)) = working_string.as_str().value_and_index_of_last_number() {
            working_string = replace_char_at_index(working_string.as_str(), index, number);
        }
        working_string
    }
    fn value_and_index_of_first_number(&self) -> Option<(char, usize)> {
        let number_array = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            "1", "2", "3", "4", "5", "6", "7", "8", "9"
        ];

        match number_array
            .into_iter()
            .enumerate()
            .filter_map(|(number_idx, number)| {
                self.find(number).map(|index| (number_idx, index))
            })
            .min_by_key(|&(_, index)| index) {
            Some((number, index)) if number < 9 => Some((number_array.get(number + 9).unwrap().parse().unwrap(), index)),
            _ => None
        }
    }
    fn value_and_index_of_last_number(&self) -> Option<(char, usize)> {
        let number_array = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            "1", "2", "3", "4", "5", "6", "7", "8", "9"
        ];

        match number_array
            .into_iter()
            .enumerate()
            .filter_map(|(number_idx, number)| {
                self.rfind(number).map(|index| (number_idx, index))
            })
            .max_by_key(|&(_, index)| index) {
            Some((number, index)) if number < 9 => Some((number_array.get(number + 9).unwrap().parse().unwrap(), index)),
            _ => None
        }
    }
    fn to_int_from_first_and_last(&self) -> i16 {
        let first = self.chars().next().unwrap().to_digit(10).unwrap() as i16;
        let last = self.chars().last().unwrap().to_digit(10).unwrap() as i16;
        first * 10 + last
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