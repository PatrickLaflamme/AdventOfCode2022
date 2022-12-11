use std::collections::LinkedList;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
pub fn generator(raw_input: &str) -> (Vec<LinkedList<char>>, Vec<(usize, usize, usize)>) {
    let (state, steps): (&str, &str) = raw_input.split("\n\n").collect_tuple().unwrap();
    let stack_count: usize = state.split("\n")
        .map(|s| s.len() + 1)
        .max()
        .unwrap() / 4;
    let mut stacks: Vec<LinkedList<char>> = (0..stack_count).map(|_| {
        LinkedList::new()
    }).collect_vec();
    state.split("\n").for_each(|row| {
        let mut i: usize = 0;
        let chars = row.chars().collect_vec();
        while i < chars.len() {
            let name: char = *chars.get(i + 1).unwrap();
            if name.is_alphabetic() {
                let stack = stacks.get_mut(i / 4).unwrap();
                stack.push_front(name);
            }
            i += 4
        }
    });
    let procedure = steps.split("\n")
        .map(|proc| {
            proc.split(" ")
                .filter(|s| s.chars().all(char::is_numeric))
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        }).collect_vec();
    (stacks, procedure)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<LinkedList<char>>, Vec<(usize, usize, usize)>)) -> String {
    let (mut state, procedure) = input.clone();
    procedure.into_iter().for_each(|(count, from, to)| {
        for _ in 0..count {
            let crate_name = state.get_mut(from - 1)
                .unwrap()
                .pop_back()
                .unwrap();
            state.get_mut(to - 1)
                .as_mut()
                .unwrap()
                .push_back(crate_name);
        }
    });
    state.into_iter().map(|list| list.back().unwrap().clone()).join("")
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<LinkedList<char>>, Vec<(usize, usize, usize)>)) -> String {
    let (mut state, procedure) = input.clone();
    procedure.into_iter().for_each(|(count, from, to)| {
        let stack = state.get_mut(from - 1)
            .unwrap();
        let mut moved = stack.split_off(stack.len() - count);
        state.get_mut(to - 1)
            .as_mut()
            .unwrap()
            .append(&mut moved);
    });
    state.into_iter().map(|list| list.back().unwrap().clone()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE: String = ["    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2"].join("\n");
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&generator(&EXAMPLE)), "CMZ");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&generator(&EXAMPLE)), "MCD");
    }
}