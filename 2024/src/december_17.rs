use itertools::Itertools;

pub fn part_1(input: &str) -> String {
    let mut cpu = CPU::from_input(input).expect("Unable to parse input to CPU");
    cpu.run();
    cpu.read_out()
}

pub fn part_2(input: &str) -> u64 {
    let mut cpu = CPU::from_input(input).expect("Unable to parse input to CPU");

    for register_a in 190593300000000..190593318000000 {
        cpu.reset();
        cpu.registers.a = register_a;
        cpu.run();
        if cpu.read_out() == cpu.read_program() {
            return register_a
        }
    }
    0
}

struct CPU {
    registers: Registers,
    instruction_pointer: u64,
    program: Vec<u64>,
    out: Vec<u64>
}

impl CPU {
    pub fn from_input(input: &str) -> Option<CPU> {
        let (registers_str, program_str) = input.split_once("\n\n")?;
        let reg_values: Vec<u64> = registers_str
            .lines()
            .map(|line|
                line.split_once(": ")
                    .expect("Register line must have colon").1
                    .parse::<u64>().expect("Register value should parse to u64")
            )
            .collect();
        let registers = Registers { a: reg_values[0], b: reg_values[1], c: reg_values[2] };
        let program: Vec<u64> = program_str
            .split_once(": ")?.1
            .trim_end()
            .split(',').map(|string| string.parse::<u64>().expect("Could not parse instruction/operand string to u32"))
            .collect();

        Some(CPU { registers, instruction_pointer: 0, program, out: Vec::new() })
    }

    fn run(&mut self) {
        while let Some(&instruction) = self.program.get(self.instruction_pointer as usize) {
            let mut should_jump = true;
            let operand = match self.program.get(self.instruction_pointer as usize + 1) {
                Some(&operand) => operand,
                None => break
            };
            match instruction {
                0 => { // adv
                    let quotient = self.registers.a / 2_u64.pow(self.combo(operand) as u32); // does this truncate?
                    self.registers.a = quotient;
                },
                1 => { // bxl
                    let bitwise_xor = self.registers.b ^ operand;
                    self.registers.b = bitwise_xor;
                },
                2 => { // bst
                    let remainder = self.combo(operand) % 8;
                    self.registers.b = remainder;
                },
                3 => { // jnz
                    match self.registers.a {
                        0 => (),
                        _ => {
                            self.instruction_pointer = operand;
                            should_jump = false;
                        }
                    }
                },
                4 => { // bxc
                    let bitwise_xor = self.registers.b ^ self.registers.c;
                    self.registers.b = bitwise_xor;
                },
                5 => { // out
                    let remainder = self.combo(operand) % 8;
                    self.out.push(remainder);
                },
                6 => { // bdv
                    let quotient = self.registers.a / 2_u64.pow(self.combo(operand) as u32); // does this truncate?
                    self.registers.b = quotient;
                },
                7 => { // cdv
                    let quotient = self.registers.a / 2_u64.pow(self.combo(operand) as u32); // does this truncate?
                    self.registers.c = quotient;
                },
                _ => unreachable!("Instruction should be 8-bit"),
            }
            if should_jump {
                self.instruction_pointer += 2;
            }
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            7 => panic!("Combo operand 7 is reserved and should not appear in valid programs."),
            _ => unreachable!("Operand should be 3-bit")
        }
    }

    fn read_out(&self) -> String {
        self.out.iter().map(|u| u.to_string()).join(",")
    }

    fn read_program(&self) -> String {
        self.program.iter().map(|u| u.to_string()).join(",")
    }

    fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.b = 0;
        self.registers.c = 0;
        self.instruction_pointer = 0;
        self.out = Vec::new();
    }
}

#[derive(Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_17_1.txt");
    assert_eq!(part_1(input), "4,6,3,5,6,3,5,2,1,0")
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_17_2.txt");
    assert_eq!(part_2(input), 117440)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_17.txt");
    assert_eq!(part_1(input), "3,1,5,3,7,4,2,7,5")
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_17.txt");
    assert_eq!(part_2(input), 190593310997519)
}
