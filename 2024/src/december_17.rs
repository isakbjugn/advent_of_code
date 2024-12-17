use itertools::Itertools;

pub fn part_1(input: &str) -> String {
    let (registers_str, program_str) = input.split_once("\n\n").expect("Should be a double line break");
    let register_values: Vec<u32> = registers_str
        .lines()
        .map(|line|
            line.split_once(": ")
                .expect("Register line must have colon").1
                .parse::<u32>().expect("Register value should parse to u64")
        )
        .collect();
    let mut registers = Registers { a: register_values[0], b: register_values[1], c: register_values[2] };
    let program: Vec<u32> = program_str
        .split_once(": ")
        .expect("Program line must have colon").1
        .trim_end()
        .split(',').map(|string| string.parse::<u32>().expect("Could not parse instruction/operand string to u32")).collect();

    let mut instruction_pointer = 0u32;
    let mut out: Vec<u32> = Vec::new();
    while let Some(&instruction) = program.get(instruction_pointer as usize) {
        let mut should_jump = true;
        let operand = match program.get(instruction_pointer as usize + 1) {
            Some(&operand) => operand,
            None => break
        };
        match instruction {
            0 => { // adv
                let quotient = registers.a / 2_u32.pow(combo(operand, &registers)); // does this truncate?
                registers.a = quotient;
            },
            1 => { // bxl
                let bitwise_xor = registers.b ^ operand;
                registers.b = bitwise_xor;
            },
            2 => { // bst
                let remainder = combo(operand, &registers) % 8;
                registers.b = remainder;
            },
            3 => { // jnz
                match registers.a {
                    0 => (),
                    _ => {
                        instruction_pointer = operand;
                        should_jump = false;
                    }
                }
            },
            4 => { // bxc
                let bitwise_xor = registers.b ^ registers.c;
                registers.b = bitwise_xor;
            },
            5 => { // out
                let remainder = combo(operand, &registers) % 8;
                out.push(remainder);
            },
            6 => { // bdv
                let quotient = registers.a / 2_u32.pow(combo(operand, &registers)); // does this truncate?
                registers.b = quotient;
            },
            7 => { // cdv
                let quotient = registers.a / 2_u32.pow(combo(operand, &registers)); // does this truncate?
                registers.c = quotient;
            },
            _ => unreachable!("Instruction should be 8-bit"),
        }
        if should_jump {
            instruction_pointer += 2;
        }
    }

    out.into_iter().map(|u| u.to_string()).join(",")
}

fn combo(operand: u32, registers: &Registers) -> u32 {
    match operand {
        0..=3 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        7 => panic!("Combo operand 7 is reserved and should not appear in valid programs."),
        _ => unreachable!("Operand should be 3-bit")
    }
}

#[derive(Debug)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_17.txt");
    assert_eq!(part_1(input), "4,6,3,5,6,3,5,2,1,0")
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_17.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_17.txt");
    assert_eq!(part_1(input), "3,1,5,3,7,4,2,7,5")
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_17.txt");
    assert_eq!(part_2(input), 0)
}
