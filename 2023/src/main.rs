mod grid;
mod sum_to;
mod direction;
mod position;

mod december_1;
mod december_2;
mod december_3;
mod december_4;
mod december_5;
mod december_5_range;
mod december_6;
mod december_7;
mod december_8;
mod december_9;
mod december_10;
mod december_11;
mod december_12;
mod december_13;
mod december_14;
mod december_15;
mod december_16;
mod december_17;
mod december_18;

use std::{
    env::args,
    fs::read_to_string
};

fn main() {
    let day = args().nth(1).unwrap();
    let path = format!("input/input_{}.txt", day);
    let input = read_to_string(path).unwrap();
    
    use std::time::Instant;
    let now = Instant::now();

    match day.as_str() {
        "1" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_1::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_1::part_2(&input));
        }
        "2" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_2::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_2::part_2(&input));
        }
        "3" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_3::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_3::part_2(&input));
        }
        "4" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_4::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_4::part_2(&input));
        }
        "5" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_5::part_1(&input));
            //println!("{}. desember, del 2: {}", day.as_str(), december_5::part_2(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_5_range::part_2_range(&input));
        }
        "6" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_6::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_6::part_2(&input));
        }
        "7" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_7::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_7::part_2(&input));
        }
        "8" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_8::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_8::part_2(&input));
        }
        "9" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_9::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_9::part_2(&input));
        }
        "10" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_10::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_10::part_2(&input));
        }
        "11" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_11::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_11::part_2(&input, 1000000));
        }
        "12" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_12::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_12::part_2(&input));
        }
        "13" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_13::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_13::part_2(&input));
        }
        "14" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_14::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_14::part_2(&input, 1000000000));
        }
        "15" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_15::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_15::part_2(&input));
        }
        "16" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_16::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_16::part_2(&input));
        }
        "17" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_17::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_17::part_2(&input));
        }
        "18" => {
            println!("{}. desember, del 1: {}", day.as_str(), december_18::part_1(&input));
            println!("{}. desember, del 2: {}", day.as_str(), december_18::part_2(&input));
        }
        _ => ()
    }

    let elapsed = now.elapsed();
    println!("Tid brukt: {:.2?}", elapsed);
}
