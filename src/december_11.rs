use std::collections::VecDeque;

pub fn part_1(input: &str) -> u32 {
    let monkey_strings = input.split("\n\n");
    let mut monkeys = Vec::<Monkey>::new();
    for monkey_string in monkey_strings {
        monkeys.push(Monkey::new(monkey_string));
    }

    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            //println!("Monkey {}:", monkey);
            for _ in 0..monkeys[monkey].items.len() {
                let mut worry_level = monkeys[monkey].inspect_item();
                //monkeys[monkey].print_inspection(worry_level);

                let true_monkey = monkeys[monkey].next_monkey.0;
                let false_monkey = monkeys[monkey].next_monkey.1;

                worry_level = monkeys[monkey].operation(worry_level);
                worry_level /= 3;

                match worry_level % monkeys[monkey].divisor {
                    0 => monkeys[true_monkey].catch_item(worry_level),
                    _ => monkeys[false_monkey].catch_item(worry_level),
                }
                monkeys[monkey].inspected += 1;
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<u32>>();
    inspected.sort();
    inspected.pop().unwrap() * inspected.pop().unwrap()
}

pub fn part_2(input: &str) -> u32 {
    2
}

struct Monkey {
    pub items: VecDeque<u32>,
    pub operator: char,
    pub operand: String,
    pub divisor: u32,
    pub next_monkey: (usize, usize),
    pub inspected: u32,
}

impl Monkey {
    pub fn new(str: &str) -> Monkey {
        let mut lines = str.split('\n');
        lines.next().unwrap();

        let (_, item_string) = lines.next().unwrap().split_once(": ").unwrap();
        let items: VecDeque<u32> = match item_string.contains(',') {
            true => item_string.split(", ").map(|s| s.parse::<u32>().unwrap()).collect::<VecDeque<u32>>(),
            false => {
                let mut new_vec_deque = VecDeque::new();
                new_vec_deque.push_back(item_string.parse::<u32>().unwrap());
                new_vec_deque
            }
        };

        let (_, operation_string) = lines.next().unwrap().split_once("= ").unwrap();
        let mut symbols = operation_string.rsplit(' ');
        let operand = String::from(symbols.next().unwrap());
        let operator = symbols.next().unwrap().chars().next().unwrap();

        let (_, divisor) = lines.next().unwrap().rsplit_once(' ').unwrap();
        let (_, true_monkey) = lines.next().unwrap().rsplit_once(' ').unwrap();
        let (_, false_monkey) = lines.next().unwrap().rsplit_once(' ').unwrap();

        Monkey {
            items,
            operator,
            operand,
            divisor: divisor.parse::<u32>().unwrap(),
            next_monkey: (true_monkey.parse::<u32>().unwrap() as usize, false_monkey.parse::<u32>().unwrap() as usize),
            inspected: 0
        }
    }

    pub fn operation(&self, old: u32) -> u32 {
        match self.operand.as_str() {
            "old" => {
                match self.operator {
                    '*' => old * old,
                    _ => old + old
                }
            }
            x => {
                match self.operator {
                    '*' => old * x.parse::<u32>().unwrap(),
                    _ => old + x.parse::<u32>().unwrap()
                }
            }
        }
    }

    pub fn inspect_item(&mut self) -> u32 {
        self.items.pop_front().unwrap()
    }

    pub fn catch_item(&mut self, item: u32) {
        self.items.push_back(item);
    }

    pub fn print_inspection(&self, worry_level: u32) {
        println!("  Monkey inspects an item with a worry level of {}.", worry_level);

        let true_monkey = self.next_monkey.0;
        let false_monkey = self.next_monkey.1;

        let mut worry_level = self.operation(worry_level);
        let operand = match self.operand.as_str() {
            "old" => worry_level,
            x => x.parse::<u32>().unwrap()
        };
        match self.operator {
            '*' => {
                println!("    Worry level is multiplied by {} to {}.", operand,  worry_level);
            },
            _ => {
                println!("    Worry level is increased by {} to {}.", operand,  worry_level);
            }
        }

        worry_level /= 3;
        println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.",  worry_level);

        match worry_level % self.divisor {
            0 => {
                println!("    Current worry level is divisible by {}.", self.divisor);
                println!("    Item with worry level {} is thrown to monkey {}.", worry_level, true_monkey);
            },
            _ => {
                println!("    Current worry level is not divisible by {}.", self.divisor);
                println!("    Item with worry level {} is thrown to monkey {}.", worry_level, false_monkey);
            },
        }
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_1(input), 10605)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_2(input), 2)
}
