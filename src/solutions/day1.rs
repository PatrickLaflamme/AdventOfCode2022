use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;

#[aoc(day1, part1)]
pub fn solve_part1(raw_input: &str) -> usize {
    raw_input.split("\n\n").map(|elf_input|{
        elf_input.split("\n")
            .map(|int_str| {
                int_str.trim().parse::<usize>().unwrap()
            }).sum()
    }).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(raw_input: &str) -> usize {
    let mut res: Vec<usize> = raw_input.split("\n\n").map(|elf_input|{
        elf_input.split("\n")
            .map(|int_str| {
                int_str.trim().parse::<usize>().unwrap()
            }).sum()
    }).sorted();
    res.reverse();
    res[0..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&EXAMPLE), 24000);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&EXAMPLE), 45000);
    }
}