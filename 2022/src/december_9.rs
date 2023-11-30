use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);

    for motion in input.lines() {
        let mut split = motion.split(' ');
        let (direction, steps) = (split.next().unwrap().chars().next().unwrap(), split.next().unwrap().parse::<u32>().unwrap());
        for _step in 0..steps {
            match direction {
                'U' => head.0 += 1,
                'D' => head.0 -= 1,
                'R' => head.1 += 1,
                'L' => head.1 -= 1,
                _ => ()
            }
            if (head.0 - tail.0 as i32).abs() <= 1 && (head.1 - tail.1 as i32).abs() == 2 {
                tail.0 = head.0;
                match head.1 - tail.1 {
                    2 => tail.1 += 1,
                    -2 => tail.1 -= 1,
                    _ => ()
                }
            }
            if (head.1 - tail.1 as i32).abs() <= 1 && (head.0 - tail.0 as i32).abs() == 2 {
                tail.1 = head.1;
                match head.0 - tail.0 {
                    2 => tail.0 += 1,
                    -2 => tail.0 -= 1,
                    _ => ()
                }
            }
            visited.insert(tail);
        }
    }

    visited.len() as u32
}

pub fn part_2(input: &str) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited.insert(rope[9]);

    for motion in input.lines() {
        let mut split = motion.split(' ');
        let (direction, steps) = (split.next().unwrap().chars().next().unwrap(), split.next().unwrap().parse::<u32>().unwrap());
        for _step in 0..steps {
            match direction {
                'U' => rope[0].0 += 1,
                'D' => rope[0].0 -= 1,
                'R' => rope[0].1 += 1,
                'L' => rope[0].1 -= 1,
                _ => ()
            }
            for knot in 1..10 {
                if (rope[knot - 1].0 - rope[knot].0 as i32).abs() <= 1 && (rope[knot - 1].1 - rope[knot].1 as i32).abs() == 2 {
                    rope[knot].0 = rope[knot - 1].0;
                    match rope[knot - 1].1 - rope[knot].1 {
                        2 => rope[knot].1 += 1,
                        -2 => rope[knot].1 -= 1,
                        _ => ()
                    }
                }
                if (rope[knot - 1].1 - rope[knot].1 as i32).abs() <= 1 && (rope[knot - 1].0 - rope[knot].0 as i32).abs() == 2 {
                    rope[knot].1 = rope[knot - 1].1;
                    match rope[knot - 1].0 - rope[knot].0 {
                        2 => rope[knot].0 += 1,
                        -2 => rope[knot].0 -= 1,
                        _ => ()
                    }
                }
                if (rope[knot - 1].1 - rope[knot].1 as i32).abs() == 2 && (rope[knot - 1].0 - rope[knot].0 as i32).abs() == 2 {
                    match rope[knot - 1].0 - rope[knot].0 {
                        2 => rope[knot].0 += 1,
                        -2 => rope[knot].0 -= 1,
                        _ => ()
                    }
                    match rope[knot - 1].1 - rope[knot].1 {
                        2 => rope[knot].1 += 1,
                        -2 => rope[knot].1 -= 1,
                        _ => ()
                    }
                }
            }

            visited.insert(rope[9]);
        }
    }

    visited.len() as u32
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_9_1.txt");
    assert_eq!(part_1(input), 13)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_9_2.txt");
    assert_eq!(part_2(input), 36)
}
