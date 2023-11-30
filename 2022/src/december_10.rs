
pub fn part_1(input: &str) -> u32 {
    let mut clock = 1;
    let mut x: i32 = 1;
    let mut clock_ticks = Vec::<u32>::new();
    let mut signal_strengths = Vec::<i32>::new();

    for line in input.lines() {
        if line == "noop" {
            if (clock + 20) % 40 == 0 {
                clock_ticks.push(clock);
                signal_strengths.push(clock as i32 * x);
            }
            clock += 1;
            continue;
        }
        let term = line.rsplit_once(' ').unwrap().1.parse::<i32>().unwrap();
        for _ in 0..2 {
            if (clock + 20) % 40 == 0 {
                clock_ticks.push(clock);
                signal_strengths.push(clock as i32 * x);
            }
            clock += 1;
        }
        x += term;
    }
    signal_strengths.iter().sum::<i32>().unsigned_abs()
}

enum Step {
    Idle,
    Adding
}

pub fn part_2(input: &str) {
    let mut x: i32 = 1;
    let mut step = Step::Idle;
    let mut lines = input.lines();
    let mut term = 0;

    /*
    loop {
        //println!("Klokke: {} ({}), x: {}", clock % 40, clock, x);
        if ((clock - 1) % 40 - x).abs() < 2 {
            print!("#");
        } else {
            print!(".")
        };
        clock += 1;

        match step {
            Step::Idle => {
                let line = lines.next().unwrap();
                match line {
                    "noop" => (),
                    _ => {
                        term = line.rsplit_once(' ').unwrap().1.parse::<i32>().unwrap();
                        step = Step::Adding;
                    }
                }
            },
            Step::Adding => {
                x += term;
                step = Step::Idle
            }
        }
        if (clock - 1) % 40 == 0 { println!(); }
    }
    */

    for _ in 0..6 {
        for pixel in 0..40 {
            if (pixel - x).abs() < 2 {
                print!("#");
            } else {
                print!(".")
            };

            match step {
                Step::Idle => {
                    match lines.next() {
                        Some(line) => {
                            match line {
                                "noop" => (),
                                _ => {
                                    term = line.rsplit_once(' ').unwrap().1.parse::<i32>().unwrap();
                                    step = Step::Adding;
                                }
                            }
                        },
                        None => break
                    }
                },
                Step::Adding => {
                    x += term;
                    step = Step::Idle
                }
            }
        }
        println!();
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_10.txt");
    assert_eq!(part_1(input), 13140)
}
