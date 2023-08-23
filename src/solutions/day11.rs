use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::mod_floor;
use regex::Regex;
use regex_split::RegexSplit;
use crate::solutions::day11::Op::{Add, Input, Int, Mul};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Vec<Op>,
    check: Check,
    inspections: u64,
}

impl Monkey {
    fn op(&self, item: &u64, lcd: &u64, worry_regularizer: &u64) -> u64 {
        let worry_pre_disinterest = {
            let left = match &self.operation[0] {
                Input => item,
                Int(val) => val,
                _ => panic!("Left component of the operation must be a value type {:?}", self.operation[0])
            };
            let right = match &self.operation[2] {
                Input => item,
                Int(val) => val,
                _ => panic!("Right component of the operation must be a value type {:?}", self.operation[0])
            };
            match &self.operation[1] {
                Mul => left * right,
                Add => left + right,
                _ => panic!("Center component of the operation must be an op type {:?}", self.operation[0])
            }
        };
        mod_floor(worry_pre_disinterest / worry_regularizer, lcd.clone())
    }

    fn check(&self, worry: &u64) -> usize {
        if mod_floor(*worry, self.check.divisor) == 0 {
            return self.check.pass_if_true;
        }
        self.check.pass_if_false
    }

    fn turn(&mut self, lcd: &u64, worry_regularizer: &u64) -> Vec<(usize, u64)> {
        self.inspections += self.items.len() as u64;
        let passes = self.items.iter().map(|item| {
            let worry = self.op(item, lcd, worry_regularizer);
            let pass_to = self.check(&worry);
            (pass_to, worry)
        }).collect_vec();
        self.items = Vec::new();
        passes
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Op {
    Input,
    Int(u64),
    Mul,
    Add
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Check {
    divisor: u64,
    pass_if_true: usize,
    pass_if_false: usize,
}

fn split_to_monkey(monkey_description: &str) -> Monkey {
    let re = Regex::new(r"\n {2}[A-Za-z]").expect("Invalid regex");
    let components = re.split_inclusive(monkey_description)
        .map(|x| { re.replace_all(x, "") })
        .collect_vec();
    assert_eq!(components.len(), 4);
    let items: Vec<u64> = components[1].split(": ")
        .last()
        .expect(&*format!("The items component must have a \": \" delimiter! {}", components[1]))
        .split(", ")
        .map(|x| { x.parse().expect(&*format!("int: [{}]", x)) })
        .collect_vec();
    let operation: Vec<Op> = components[2].split(": ")
        .last()
        .expect(&*format!("The operation component must have a \": \" delimiter! {}", components[2]))
        .split(" = ")
        .last()
        .expect(&*format!("The operation component equation must have a \" = \" delimiter! {}", components[2]))
        .split(" ")
        .map(|x| {
            match x {
                "old" => Input,
                "*" => Mul,
                "+" => Add,
                int => Int(int.parse().unwrap())
            }
        }).collect_vec();
    let check_components: Vec<usize> = components[3].split("\n")
        .map(|x| {
            x.split(" ")
                .last()
                .expect(&*format!("Each line of the check section must have characters! {}", components[3]))
                .parse()
                .expect(&*format!("Each line of the check section must end with an int! {}", components[3]))
        }).collect_vec();
    let check: Check = Check {
        divisor: check_components[0] as u64,
        pass_if_true: check_components[1],
        pass_if_false: check_components[2]
    };
    Monkey {
        items,
        operation,
        check,
        inspections: 0,
    }
}

#[aoc_generator(day11)]
pub fn generator(raw_input: &str) -> Vec<Monkey> {
    raw_input.split("\n\n")
        .into_iter()
        .map(split_to_monkey)
        .collect_vec()
}

fn simulate(monkeys_reading: &Vec<Monkey>, iterations: usize, worry_regularizer: u64) -> u64 {
    let monkeys: &mut Vec<Monkey> = &mut monkeys_reading.clone();
    let lcd = monkeys_reading.iter().fold(1, |acc, m| {
        acc * m.check.divisor
    });
    for _ in 0..iterations {
        for i in 0..monkeys_reading.len() {
            let m = monkeys.get_mut(i).expect("");
            m.turn(&lcd, &worry_regularizer).iter()
                .foreach(|(to, worry)| {
                    let to_m = monkeys.get_mut(to.clone()).expect("");
                    to_m.items.push(*worry);
                });
        }
    }
    let mut top: u64 = 0;
    let mut second: u64 = 0;
    monkeys.iter()
        .for_each(|monkey| {
            if top <= monkey.inspections {
                second = top;
                top = monkey.inspections;
            } else if second < monkey.inspections {
                second = monkey.inspections;
            }
        });
    top * second
}

#[aoc(day11, part1)]
pub fn solve_part1(monkeys_reading: &Vec<Monkey>) -> u64 {
    simulate(monkeys_reading, 20, 3)
}

#[aoc(day11, part2)]
pub fn solve_part2(monkeys_reading: &Vec<Monkey>) -> u64 {
    simulate(monkeys_reading, 10000, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_generator() {
        let expected = vec![
            Monkey {
                items: vec![79, 98],
                operation: vec![Input, Mul, Int(19)],
                check: Check {
                    divisor: 23,
                    pass_if_true: 2,
                    pass_if_false: 3
                },
                inspections: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: vec![Input, Add, Int(6)],
                check: Check {
                    divisor: 19,
                    pass_if_true: 2,
                    pass_if_false: 0
                },
                inspections: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: vec![Input, Mul, Input],
                check: Check {
                    divisor: 13,
                    pass_if_true: 1,
                    pass_if_false: 3
                },
                inspections: 0,
            },
            Monkey {
                items: vec![74],
                operation: vec![Input, Add, Int(3)],
                check: Check {
                    divisor: 17,
                    pass_if_true: 0,
                    pass_if_false: 1
                },
                inspections: 0,
            },
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<Monkey> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 10605);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Monkey> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 2713310158);
    }
}