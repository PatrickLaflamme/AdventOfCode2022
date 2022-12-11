use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn contains_other(sself: &RangeInclusive<usize>, other: &RangeInclusive<usize>) -> bool {
    sself.contains(other.start()) && sself.contains(other.end())
}

fn overlaps_other(sself: &RangeInclusive<usize>, other: &RangeInclusive<usize>) -> bool {
    sself.contains(other.start()) || sself.contains(other.end())
}

#[aoc_generator(day4)]
pub fn generator(raw_input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    raw_input.split("\n")
        .map(|s| {
            let (first, second) = s.trim()
                .split(",")
                .map(|e| {
                    let (start, end) = e.split("-")
                        .map(|b| b.parse().unwrap())
                        .collect_tuple().unwrap();
                    RangeInclusive::new(start, end)
                }).collect_tuple().unwrap();
            (first, second)
        }).collect_vec()
}

#[aoc(day4, part1)]
pub fn solve_part1(plan: &[(RangeInclusive<usize>, RangeInclusive<usize>)]) -> usize {
    plan.iter()
        .filter(|&(first, second)| {
            contains_other(first, second) || contains_other(second, first)
        }).collect_vec().len()
}

#[aoc(day4, part2)]
pub fn solve_part2(plan: &[(RangeInclusive<usize>, RangeInclusive<usize>)]) -> usize {
    plan.iter()
        .filter(|&(first, second)| {
            overlaps_other(first, second) || overlaps_other(second, first)
        }).collect_vec().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&generator(&EXAMPLE)), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&generator(&EXAMPLE)), 4);
    }
}