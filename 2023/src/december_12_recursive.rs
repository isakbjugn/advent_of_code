use std::collections::HashMap;

pub fn part_1(input: &str) -> u64 {
    input.lines()
        .map(to_record_and_groups)
        .map(|(record, groups)| search(record, &groups, &mut HashMap::new()))
        .sum()
}

fn search(record: &str, groups: &Vec<u32>, memo: &mut HashMap<(String, u32), u64>) -> u64 {
    let defect_springs = groups.iter().sum::<u32>();
    if let Some(&result) = memo.get(&(String::from(record), defect_springs)) {
        return result
    }

    // println!("Undersøker {} mot {:?}", record, groups);
    if record.is_empty() && groups.is_empty() {
        // println!("✅"); // ✅ eller ❌
        memo.insert((String::from(record), defect_springs), 1);
        return 1 
    }
    if groups.is_empty() && !record.chars().any(|c| c == '#') {
        // println!("✅"); // ✅ eller ❌
        memo.insert((String::from(record), defect_springs), 1);
        return 1
    }
    if groups.first() == Some(&0) {
        let result = search(record, &groups.iter().skip(1).cloned().collect(), memo);
        memo.insert((String::from(record), defect_springs), result);
        return result
    }
    if record.is_empty() {
        memo.insert((String::from(record), defect_springs), 0);
        return 0
    }
    if groups.is_empty() {
        memo.insert((String::from(record), defect_springs), 0);
        return 0
    }
    if (record == "#" || record == "?") && groups.len() == 1 && groups.first() == Some(&1) {
        // println!("✅");
        memo.insert((String::from(record), defect_springs), 1);
        return 1
    }
    
    if record.chars().filter(|&c| c == '#').count() > defect_springs as usize {
        memo.insert((String::from(record), defect_springs), 0);
        return 0
    }
    
    let minimum_remaining_length = groups.iter().sum::<u32>() as usize + groups.len() - 1;
    if record.len() < minimum_remaining_length {
        memo.insert((String::from(record), defect_springs), 0);
        return 0
    }

    let first_group = match groups.first() {
        Some(0) => {
            let result = search(record, &groups.iter().skip(1).cloned().collect(), memo);
            memo.insert((String::from(record), defect_springs), result);
            return result
        },
        Some(&n) => n,
        None => {
            memo.insert((String::from(record), defect_springs), 0);
            return 0
        }
    };
    
    let new_groups = [first_group - 1].iter().chain(groups.iter().skip(1)).cloned().collect();

    let result = match record.split_at_checked(1) {
        None => 0,
        Some(("#", remainder)) => {
            match record.split_at_checked(2) {
                Some(("##", _)) if groups.first() == Some(&1) => {
                    0
                },
                Some(("#.", _)) if groups.first().unwrap() > &1 => {
                    0
                },
                Some(("#?", new_remainder)) if groups.first() == Some(&1) => {
                    search(&format!("#.{}", new_remainder), groups, memo)
                },
                Some(("#?", new_remainder)) if groups.first().unwrap() > &1 => {
                    search(&format!("##{}", new_remainder), groups, memo)
                }
                _ => search(remainder, &new_groups, memo)
            }
        },
        Some((".", remainder)) => search(remainder, groups, memo),
        Some(("?", remainder)) => {
            match record.split_at_checked(2) {
                Some(("??", new_remainder)) if groups.first() == Some(&1) => {
                    search(&format!("#.{}", new_remainder), groups, memo) + search(&format!(".{}", remainder), groups, memo)
                },
                Some(("?#", new_remainder)) if groups.first() == Some(&1) => {
                    search(&format!(".#{}", new_remainder), groups, memo)
                },
                _ => {
                    search(&format!("#{}", remainder), groups, memo) + search(&format!(".{}", remainder), groups, memo)
                }
            }
        },
        _ => panic!("Ulovlig tegn!")
    };

    memo.insert((String::from(record), defect_springs), result);
    result
}

pub fn part_2(input: &str) -> u64 {
    input.lines()
        .map(to_long_format)
        .map(to_record_and_groups)
        .map(|(record, groups)| search(record, &groups, &mut HashMap::new()))
        .sum()
}

fn to_record_and_groups(line: &str) -> (&str, Vec<u32>) {
    let (record, groups) = line.split_once(' ').unwrap();
    (record,groups.split(',').map(|c| c.parse::<u32>().unwrap()).collect())
}

fn to_long_format(input: &str) -> &str {
    let (record, groups) = input.split_once(' ').unwrap();
    let mut long_format = String::from(input);
    for _ in 1..5 {
        long_format = format!("{}?{},{}", record, long_format, groups);
    }
    Box::leak(long_format.into_boxed_str())
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_1(input), 21)
}

#[test]
fn sample_input_part_1_1() {
    assert_eq!(part_1("???.### 1,1,3"), 1);
    assert_eq!(part_1(".??..??...?##. 1,1,3"), 4);
    assert_eq!(part_1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(part_1("????.#...#... 4,1,1"), 1);
    assert_eq!(part_1("????.######..#####. 1,6,5"), 4);
    assert_eq!(part_1("?###???????? 3,2,1"), 10);
}

#[test]
fn specific_line_part_1() {
    assert_eq!(part_1("..??#???##??#?? 4,2,2"), 4);
    assert_eq!(part_1(".#?????????.?. 9,1"), 1);
    assert_eq!(part_1(".????#..??#? 4,2"), 2);
    assert_eq!(part_1("??#.#???#? 2,1,1"), 1);
    assert_eq!(part_1("?#??.???#?#????? 4,1,1,2,3"), 2);
    assert_eq!(part_1(".??????.?##. 1,3"), 6);
}

#[test]
fn actual_input_part_1() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_1(input), 7221)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_2(input), 525152)
}