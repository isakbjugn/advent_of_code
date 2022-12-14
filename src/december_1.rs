
pub fn part_1(input: &str) -> i32 {
  let lines = input.lines();
  let mut calories: Vec<i32> = Vec::new();
  let mut sum = 0;

  for str in lines {
    if str.is_empty() && sum != 0 {
      calories.push(sum);
      sum = 0;
    }
    else {
      sum += str.parse::<i32>().unwrap();
    }
  }
  calories.push(sum);
  calories.into_iter().max().unwrap()
}

pub fn part_2(input: &str) -> i32 {
  let lines = input.lines();
  let mut calories: Vec<i32> = Vec::new();
  let mut sum = 0;

  for str in lines {
    if str.is_empty() && sum != 0 {
      calories.push(sum);
      sum = 0;
    }
    else {
      sum += str.parse::<i32>().unwrap();
    }
  }
  calories.push(sum);

  calories.sort();
  calories.reverse();
  calories.iter().take(3).sum()
}

#[test]
fn sample_input_part_1() {
  let input = include_str!("../input/sample_1.txt");
  assert_eq!(part_1(input), 24000)
}

#[test]
fn sample_input_part_2() {
  let input = include_str!("../input/sample_1.txt");
  assert_eq!(part_2(input), 45000)
}