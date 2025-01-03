use std::{env::args, fs::read_to_string};
use took::Timer;

mod position;
mod direction;
mod grid;
mod sliceable;
mod with;

mod december_1;
mod december_2;
mod december_3;
mod december_4;
mod december_5;
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
mod december_19;
mod december_20;
mod december_21;
mod december_22;
mod december_23;
mod december_24;
mod december_25;

fn main() {
    let day = args().nth(1).unwrap();
    let path = format!("input/input_{}.txt", day);
    let input = read_to_string(path).unwrap();

    let timer = Timer::new();
    #[cfg_attr(rustfmt, rustfmt_skip)]
    match day.as_str() {
        "1" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_1::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_1::part_2(&input));
        }
        "2" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_2::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_2::part_2(&input));
        }
        "3" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_3::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_3::part_2(&input));
        }
        "4" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_4::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_4::part_2(&input));
        }
        "5" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_5::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_5::part_2(&input));
        }
        "6" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_6::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_6::part_2(&input));
        }
        "7" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_7::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_7::part_2(&input));
        }
        "8" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_8::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_8::part_2(&input));
        }
        "9" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_9::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_9::part_2(&input));
        }
        "10" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_10::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_10::part_2(&input));
        }
        "11" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_11::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_11::part_2(&input));
        }
        "12" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_12::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_12::part_2(&input));
        }
        "13" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_13::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_13::part_2(&input));
        }
        "14" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_14::part_1(&input, 103, 101));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_14::part_2(&input, 103, 101));
        }
        "15" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_15::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_15::part_2(&input));
        }
        "16" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_16::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_16::part_2(&input));
        }
        "17" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_17::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_17::part_2(&input));
        }
        "18" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_18::part_1(&input, 71, 71, 1024));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_18::part_2(&input, 71, 71, 1024));
        }
        "19" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_19::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_19::part_2(&input));
        }
        "20" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_20::part_1(&input, 100));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_20::part_2(&input, 100));
        }
        "21" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_21::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_21::part_2(&input));
        }
        "22" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_22::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_22::part_2(&input));
        }
        "23" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_23::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_23::part_2(&input));
        }
        "24" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_24::part_1(&input));
            println!("{}. desember, del 2: {:?}", day.as_str(), december_24::part_2(&input));
        }
        "25" => {
            println!("{}. desember, del 1: {:?}", day.as_str(), december_25::part_1(&input));
        }
        _ => ()
    }
    println!("Finished in {}", timer.took());
}
