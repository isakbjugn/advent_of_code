mod december_1;
mod december_2;

use std::{
    env::args,
    fs::read_to_string
};

fn main() {
    let day = args().nth(1).unwrap();
    let path = format!("input/input_{}.txt", day);
    let input = read_to_string(path).unwrap();

    match day.as_str() {
        "1" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_1::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_1::part_2(&input));
        }
        "2" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_2::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_2::part_2(&input));
        }
        _ => ()
    }
}