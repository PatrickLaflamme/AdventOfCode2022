use std::collections::{HashMap, LinkedList};
use aoc_runner_derive::aoc;

fn find_first_unique_sequence(input: &str, sequence_length: usize) -> usize {
    let mut window: LinkedList<char> = LinkedList::new();
    let mut seen: HashMap<char, usize> = HashMap::new();
    for (i, c) in input.chars().enumerate() {
        while window.len() >= sequence_length {
            let remove = window.pop_front().unwrap();
            let count = *seen.get_mut(&remove).unwrap();
            if count <= 1 {
                seen.remove(&remove);
            } else {
                seen.insert(remove, count - 1);
            }
        }
        window.push_back(c);
        seen.insert(c, seen.get(&c).unwrap_or(&0) + 1);
        if seen.len() == sequence_length {
            return i + 1;
        }
    }
    panic!("No valid start sequence in [{:?}]", String::from(input));
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    find_first_unique_sequence(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    find_first_unique_sequence(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[(&str, usize, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26)
    ];

    #[test]
    fn test_solve_part1() {
        for &(example, expected, _) in EXAMPLES {
            assert_eq!(solve_part1(example), expected);
        }
    }

    #[test]
    fn test_solve_part2() {
        for &(example, _, expected) in EXAMPLES {
            assert_eq!(solve_part2(example), expected);
        }
    }
}