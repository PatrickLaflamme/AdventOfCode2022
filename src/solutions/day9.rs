use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use num::{abs, signum};

lazy_static! {
    static ref d_map: HashMap<char, (i16, i16)> = HashMap::from([
        ('D', (0,-1)),
        ('U', (0,1)),
        ('R', (1,0)),
        ('L', (-1,0))
    ]);
}

fn move_head_knot(knot: (i16, i16), direction: &char) -> (i16, i16) {
    let movement = d_map.get(direction).unwrap();
    return (knot.0 + movement.0, knot.1 + movement.1);
}

fn move_trailing_knot(
    prev_knot: &(i16, i16),
    knot: (i16, i16)
) -> (i16, i16) {
    if !causes_movement(*prev_knot, &knot) {
        return knot;
    }
    let head_movement = (signum(prev_knot.0 - knot.0), signum(prev_knot.1 - knot.1));
    return (knot.0 + head_movement.0, knot.1 + head_movement.1);
}

fn causes_movement(new_knot_position: (i16, i16), next_knot: &(i16, i16)) -> bool {
    let x_stretch = abs(new_knot_position.0 - next_knot.0) > 1;
    let y_stretch = abs(new_knot_position.1 - next_knot.1) > 1;
    return x_stretch || y_stretch;
}

fn move_rope(rope: &Vec<(i16, i16)>, head_movement: &char) -> Vec<(i16, i16)> {
    let mut new_rope_position: Vec<(i16, i16)> = Vec::with_capacity(rope.len());
    new_rope_position.insert(0, move_head_knot(rope[0], head_movement));
    for i in 1..rope.len() {
        let knot = &rope[i];
        let prev_knot = &new_rope_position[i-1];
        let knot_new_pos = move_trailing_knot(prev_knot, *knot);
        new_rope_position.insert(i, knot_new_pos);
    }
    assert_eq!(rope.len(), new_rope_position.len());
    new_rope_position
}

fn simulate_rope(movements: &Vec<(char, i16)>, rope_length: usize) -> usize {
    let mut rope: Vec<(i16, i16)> = (0..rope_length).map(|_| {(0,0)}).collect_vec();
    let mut t_locs: HashSet<(i16, i16)> = HashSet::from([
        rope.last().unwrap().clone()
    ]);
    movements.iter().foreach(|(d, c)| {
        for _ in 0..*c {
            rope = move_rope(&rope, d);
            t_locs.insert(rope.last().unwrap().clone());
        }
    });
    t_locs.len()
}

#[aoc_generator(day9)]
pub fn generator(raw_input: &str) -> Vec<(char, i16)> {
    raw_input.split("\n").map(|r| {
        let (d_str, c_str): (&str, &str) = r.split(" ").collect_tuple().unwrap();
        (d_str.chars().into_iter().rev().last().unwrap(), c_str.parse().unwrap())
    }).collect_vec()
}

#[aoc(day9, part1)]
pub fn solve_part1(movements: &Vec<(char, i16)>) -> usize {
    simulate_rope(movements, 2)
}

#[aoc(day9, part2)]
pub fn solve_part2(movements: &Vec<(char, i16)>) -> usize {
    simulate_rope(movements, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_generator() {
        let expected = vec![
            ('R', 4),
            ('U', 4),
            ('L', 3),
            ('D', 1),
            ('R', 4),
            ('D', 1),
            ('L', 5),
            ('R', 2),
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<(char, i16)> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 13);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<(char, i16)> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 1);
    }

    #[test]
    fn test2_solve_part2() {
        let example: Vec<(char, i16)> = generator(&EXAMPLE2);
        assert_eq!(solve_part2(&example), 36);
    }
}