use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

static SCORES: &'static [(&'static str, u8)] = &[
    ("A", 0),
    ("B", 1),
    ("C", 2),
    ("X", 0),
    ("Y", 1),
    ("Z", 2)
];
static REM: u8 = 3;

fn decode_action(action: &str) -> &'static u8 {
    for s in SCORES {
        if s.0 == action {
            return &s.1
        }
    }
    panic!("Invalid Action: [{}]", action)
}

fn score_round(other_action: &u8, self_action: &u8) -> u8 {
    let other_losing_action = &(other_action + 1).rem_euclid(REM);
    return if self_action == other_losing_action {
        6
    } else if self_action == other_action {
        3
    } else {
        0
    }
}

fn choose_action(other_action: &u8, required_outcome: &u8) -> u8 {
    if required_outcome == &0 {
        if other_action == &0 {
            2
        } else {
            other_action - 1
        }
    } else if required_outcome == &1 {
        *other_action
    } else {
        (other_action + 1).rem_euclid(REM)
    }
}

#[aoc_generator(day2)]
pub fn generator(raw_input: &str) -> Vec<(&'static u8, &'static u8)> {
    raw_input.split("\n").map(|round| {
        let split = round.trim().split(" ").collect_vec();
        (decode_action(split[0]), decode_action(split[1]))
    }).collect_vec()
}

#[aoc(day2, part1)]
pub fn solve_part1(rounds: &[(&u8, &u8)]) -> usize {
    rounds.into_iter().map(|round| {
        (round.1 + 1) as usize + score_round(round.0, round.1) as usize
    }).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(rounds: &[(&u8, &u8)]) -> usize {
    rounds.into_iter().map(|round| {
        let action = choose_action(round.0, round.1);
        (action + 1) as usize + (round.1 * 3) as usize
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
        B X
        C Z";

    #[test]
    fn test_generator() {
        let expected = &[
            (&0,&1),
            (&1,&0),
            (&2,&2)
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<(&u8, &u8)> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 15);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<(&u8, &u8)> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 12);
    }
}