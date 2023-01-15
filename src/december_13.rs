use std::cmp::Ordering;

pub fn part_1(input: &str) -> u32 {
    let mut pair_index_sum = 0;
    for (idx, pair) in input.split("\n\n").into_iter().enumerate() {
        //println!("== Par {} ==", idx + 1);
        let mut split_pair = pair.split('\n');
        match compare(split_pair.next().unwrap(), split_pair.next().unwrap(), "") {
            Ordering::Greater => {
                //println!("Paret har feil rekkefølge!");
            },
            _ => {
                //println!("Paret har riktig rekkefølge!");
                pair_index_sum += idx as u32 + 1;
            }
        }
        //println!();
    }
    pair_index_sum
}

pub fn part_2(_input: &str) -> u32 {
    let mut packets = Vec::new();
    for line in _input.lines() {
        if line.is_empty() { continue }
        packets.push(line);
    }
    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|&a, &b| compare(a, b, ""));

    let index_2 = packets.iter().position(|&x| x == "[[2]]").unwrap() as u32;
    let index_6 = packets.iter().position(|&x| x == "[[6]]").unwrap() as u32;
    (index_2 + 1) * (index_6 + 1)
}

fn compare(left: &str, right: &str, indent: &str) -> Ordering {
    //println!("{}Samanliknar {} og {}", indent, left, right);
    let indent = &format!("{}{}", indent, "  ")[..];

    // Sjekk om nokon av strengane er tomme
    match (left.is_empty(), right.is_empty()) {
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        _ => ()
    }

    // Begge er heiltal
    if let (Ok(l), Ok(r)) = (left.parse::<u32>(), right.parse::<u32>()) {
        return l.cmp(&r)
    }

    // Venstre er heiltal, høgre er liste
    if let Ok(l) = left.parse::<u32>() {
        return compare(&format!("[{}]", l)[..], right, indent)
    }

    // Høgre er heiltal, venstre er liste
    if let Ok(r) = right.parse::<u32>() {
        return compare(left, &format!("[{}]", r)[..], indent)
    }

    // Begge er lister
    let left_substr = split_list_by_commas(left);
    let right_substr = split_list_by_commas(right);

    // Sjekk om nokon av listene er tomme
    match (left_substr.first().unwrap().is_empty(), right_substr.first().unwrap().is_empty()) {
        (true, true) => return Ordering::Equal,
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        (false, false) => ()
    }

    for idx in 0..left_substr.len() {
        // Sjekk om høgre liste er tom for element
        if idx == right_substr.len() {
            return Ordering::Greater
        }
        match compare(left_substr[idx], right_substr[idx], indent) {
            Ordering::Equal => continue,
            x => return x
        }
    }

    // Om det enno er fleire element i høgre liste, da er ho støre
    match right_substr.len() > left_substr.len() {
        true => {
            Ordering::Less
        },
        false => Ordering::Equal
    }
}

fn inner_str(str: &str) -> &str {
    &str[1..str.len()-1]
}

fn split_list_by_commas(str: &str) -> Vec<&str> {
    let inner_str = inner_str(str);
    let commas = find_outer_commas(inner_str);
    if commas.is_empty() {
        return vec![inner_str];
    }

    let mut sub_strings = Vec::new();
    for i in 0..commas.len() {
        if i == 0 {
            sub_strings.push(&inner_str[0..commas[i]]);
        }
        else {
            sub_strings.push(&inner_str[commas[i - 1]+1..commas[i]])
        }
    }
    sub_strings.push(&inner_str[*commas.last().unwrap()+1..inner_str.len()]);
    sub_strings
}

fn find_outer_commas(str: &str) -> Vec<usize> {
    let mut commas = Vec::new();
    let mut open_lists = 0;
    for (idx, letter) in str.chars().enumerate() {
        if letter == ',' && open_lists == 0 { commas.push(idx) }
        if letter == '[' { open_lists += 1 }
        if letter == ']' { open_lists -= 1 }
    }
    commas
}

#[test]
fn test_find_outer_commas() {
    let input = inner_str("[[4,4],4,4]");
    assert_eq!(find_outer_commas(input), [5, 7])
}

#[test]
fn test_split_list_by_commas() {
    let input = "[[4,4],4,4]";
    assert_eq!(split_list_by_commas(input), ["[4,4]", "4", "4"])
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_1(input), 13)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_13_2.txt");
    assert_eq!(part_1(input), 25)
}

#[test]
fn sample_input_part_1_3() {
    let input = include_str!("../input/sample_13_3.txt");
    assert_eq!(part_1(input), 3)
}

#[test]
fn sample_input_part_1_4() {
    let input = include_str!("../input/hard_ones_13.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn real_input_part_1_3() {
    let input = include_str!("../input/input_13.txt");
    assert_eq!(part_1(input), 6395)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_2(input), 140)
}
