
pub fn part_1(input: &str) -> u32 {
    input.lines()
        .map(to_record_and_groups)
        .map(|(record, groups)| search(record, &groups))
        .sum()
}

fn search(record: &str, groups: &Vec<u32>) -> u32 {

    // println!("Undersøker {} mot {:?}", record, groups);    
    if record.is_empty() && groups.is_empty() {
        println!("✅"); // ✅ eller ❌
        return 1 
    }
    if groups.is_empty() && !record.chars().any(|c| c == '#') {
        println!("✅"); // ✅ eller ❌
        return 1
    }
    if groups.first() == Some(&0) { return search(record, &groups.iter().skip(1).cloned().collect() ) }
    if record.is_empty() { return 0 }
    if groups.is_empty() { return 0 }
    if (record == "#" || record == "?") && groups.len() == 1 && groups.first() == Some(&1) {
        println!("✅");
        return 1
    }
    
    let defect_springs = groups.iter().sum::<u32>() as usize;
    
    if record.chars().filter(|&c| c == '#').count() > defect_springs { return 0 }
    
    let minimum_remaining_length = groups.iter().sum::<u32>() as usize + groups.len() - 1;
    if record.len() < minimum_remaining_length { return 0 }

    let first_group = match groups.first() {
        Some(0) => return search(record, &groups.iter().skip(1).cloned().collect()),
        Some(&n) => n,
        None => return 0
    };
    
    let new_groups = [first_group - 1].iter().chain(groups.iter().skip(1)).cloned().collect();

    match record.split_at_checked(1) {
        None => 0,
        Some(("#", remainder)) => {
            match record.split_at_checked(2) {
                Some(("##", _)) if groups.first() == Some(&1) => {
                    0
                },
                Some(("#?", new_remainder)) if groups.first() == Some(&1) => {
                    search(&format!("#.{}", new_remainder), groups)
                },
                Some(("#?", new_remainder)) if groups.first().unwrap() > &1 => {
                    search(&format!("##{}", new_remainder), groups)
                }
                _ => search(remainder, &new_groups)
            }
        },
        Some((".", remainder)) => search(remainder, groups),
        Some(("?", remainder)) => {
            match record.split_at_checked(2) {
                Some(("??", new_remainder)) if groups.first() == Some(&1) => {
                    search(&format!("#.{}", new_remainder), groups) + search(&format!(".{}", remainder), groups)
                },
                Some(("?#", new_remainder)) if groups.first() == Some(&1) => {
                    search(&format!(".#{}", new_remainder), groups)
                },
                _ => {
                    search(&format!("#{}", remainder), groups) + search(&format!(".{}", remainder), groups)
                }
            }
        },
        _ => panic!("Ulovlig tegn!")
    }
}

pub fn part_2(_input: &str) -> u32 {

    0
}

fn to_record_and_groups(line: &str) -> (&str, Vec<u32>) {
    let (record, groups) = line.split_once(' ').unwrap();
    (record,groups.split(',').map(|c| c.parse::<u32>().unwrap()).collect())
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_1(input), 21)
}

#[test]
fn sample_input_part_1_1() {
    assert_eq!(search("???.###", &vec![1,1,3]), 1);
    assert_eq!(search(".??..??...?##.", &vec![1,1,3]), 4);
    assert_eq!(search("?#?#?#?#?#?#?#?", &vec![1,3,1,6]), 1);
    assert_eq!(search("????.#...#...", &vec![4,1,1]), 1);
    assert_eq!(search("????.######..#####.", &vec![1,6,5]), 4);
    assert_eq!(search("?###????????", &vec![3,2,1]), 10);
}

#[test]
fn actual_input_part_1() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_1(input), 7221)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_2(input), 0)
}