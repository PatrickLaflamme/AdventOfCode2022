use std::collections::HashSet;
use std::iter::FromIterator;

use aoc_runner_derive::aoc;
use itertools::Itertools;

fn prioritize_item(item: char) -> usize {
    if item.is_ascii_uppercase() {
        item as usize - 38
    } else {
        item as usize - 96
    }
}

fn identify_duplicate_item(pack: &str) -> Option<char> {
    let mut seen: HashSet<char> = HashSet::new();
    let mut index = 0;
    let halfway = pack.len() / 2;
    for char in pack.chars() {
        if index < halfway {
            seen.insert(char);
        } else if seen.contains(&char) {
            return Some(char);
        }
        index += 1
    }
    println!("No item is present in both halves of the pack: [{}]", pack);
    None
}

fn find_badge(packs: &[&str]) -> Option<char> {
    let result = packs.into_iter()
        .map(|pack| {
            let set: HashSet<char> = HashSet::from_iter(pack.chars());
            set
        })
        .rfold(HashSet::new(), |mut acc, set| {
            if acc.len() == 0 {
                set.into_iter().foreach(|c| {
                    acc.insert(c);
                });
            } else {
                acc.retain(|c| set.contains(c));
            }
            acc
        });
    if result.len() == 1 {
        Some(result.into_iter().collect_vec().first().unwrap().clone())
    } else {
        println!("Invalid input! Found the following dupes across all packs: {:?}", result);
        None
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(raw_input: &str) -> usize {
    raw_input.split("\n")
        .map(|r| { r.trim() })
        .filter_map(identify_duplicate_item)
        .map(prioritize_item)
        .sum::<usize>()
}

#[aoc(day3, part2)]
pub fn solve_part2(raw_input: &str) -> usize {
    let groups = raw_input.split("\n")
        .map(|r| { r.trim() })
        .chunks(3);
    let mut sum = 0;
    for group in &groups {
        let badge = find_badge(&group.collect_vec()).unwrap();
        sum += prioritize_item(badge);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_prioritize_item() {
        assert_eq!(prioritize_item('a'), 1);
        assert_eq!(prioritize_item('A'), 27);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(EXAMPLE), 157);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(EXAMPLE), 70);
    }
}