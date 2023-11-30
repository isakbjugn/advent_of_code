use std::{
    env::args,
    fs::read_to_string
};

fn main() {
    let day = args().nth(1).unwrap();
    let path = format!("input/input_{}.txt", day);
    let _input = read_to_string(path).unwrap();

    match day.as_str() {
        "1" => {
            println!("{}. desember, del 1: {}", day.as_str(), "svar"); //december_1::part_1(&input)
            println!("{}. desember, del 2: {}", day.as_str(), "svar"); //december_1::part_2(&input)
        }
        _ => ()
    }
}
