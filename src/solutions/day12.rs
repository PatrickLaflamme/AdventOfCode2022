use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools};

const DIRS: [(isize, isize); 4] = [
    (1,0),
    (0,1),
    (-1,0),
    (0,-1)
];

fn add(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    return (a.0 + b.0, a.1 + b.1);
}

fn is_valid(map: &Vec<Vec<u8>>, loc: &(isize, isize)) -> bool {
    return loc.0 >= 0 && loc.1 >= 0 && loc.0 < map.len() as isize && loc.1 < map.first().unwrap().len() as isize;
}

fn get_value_at_loc(map: &Vec<Vec<u8>>, loc: &(isize, isize)) -> u8 {
    let row: usize = loc.0.try_into().expect(&*format!("{}", loc.0));
    let col: usize = loc.1.try_into().expect(&*format!("{}", loc.1));
    return map[row][col];
}

#[aoc_generator(day12)]
pub fn generator(raw_input: &str) -> ((isize, isize), (isize, isize), Vec<Vec<u8>>) {
    let mut start: Option<(isize, isize)> = None;
    let mut end: Option<(isize, isize)> = None;
    let v = raw_input.split("\n")
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, chr)| {
                    if chr == 'S' {
                        start = Some((i as isize, j as isize));
                        return 'a' as u8;
                    } else if chr == 'E' {
                        end = Some((i as isize, j as isize));
                        return 'z' as u8;
                    }
                    chr as u8
                })
                .collect_vec()
        })
        .collect_vec();
    (start.expect(""), end.expect(""), v)
}

fn shortest_distance(start: &(isize, isize), end: &(isize, isize), map: &Vec<Vec<u8>>) -> usize {
    let cap = map.len() * map.first().expect("").len();
    let mut queue = VecDeque::with_capacity(cap);
    let mut seen = HashSet::with_capacity(cap);
    seen.insert(start.clone());
    queue.push_back((0, start.clone()));
    while !queue.is_empty() {
        let (d, loc) = queue.pop_front().unwrap();
        if loc == *end {
            return d ;
        }
        for dir in DIRS {
            let new_loc = add(&loc, &dir);
            if is_valid(map, &new_loc) & !seen.contains(&new_loc) {
                let v = get_value_at_loc(map, &loc);
                let new_v = get_value_at_loc(map, &new_loc);
                if new_v <= v + 1 {
                    seen.insert(new_loc.clone());
                    queue.push_back((d + 1, new_loc));
                }
            }
        }
    }
    return 0;
}

fn shortest_distance_rev(start: &(isize, isize), map: &Vec<Vec<u8>>) -> usize {
    let target: u8 = 'a'.try_into().unwrap();
    let cap = map.len() * map.first().expect("").len();
    let mut queue = VecDeque::with_capacity(cap);
    let mut seen = HashSet::with_capacity(cap);
    seen.insert(start.clone());
    queue.push_back((0, start.clone()));
    while !queue.is_empty() {
        let (d, loc) = queue.pop_front().unwrap();
        let v = get_value_at_loc(map, &loc);
        if v == target {
            return d ;
        }
        for dir in DIRS {
            let new_loc = add(&loc, &dir);
            if is_valid(map, &new_loc) & !seen.contains(&new_loc) {
                let new_v = get_value_at_loc(map, &new_loc);
                if new_v >= v - 1 {
                    seen.insert(new_loc.clone());
                    queue.push_back((d + 1, new_loc));
                }
            }
        }
    }
    return 0;
}

#[aoc(day12, part1)]
pub fn solve_part1(readings: &((isize, isize), (isize, isize), Vec<Vec<u8>>)) -> usize {
    shortest_distance(&readings.0, &readings.1, &readings.2)
}

#[aoc(day12, part2)]
pub fn solve_part2(readings: &((isize, isize), (isize, isize), Vec<Vec<u8>>)) -> usize {
    shortest_distance_rev(&readings.1, &readings.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_generator() {
        let v = vec![
            vec![97, 97, 98, 113, 112, 111, 110, 109],
            vec![97, 98, 99, 114, 121, 120, 120, 108],
            vec![97, 99, 99, 115, 122, 122, 120, 107],
            vec![97, 99, 99, 116, 117, 118, 119, 106],
            vec![97, 98, 100, 101, 102, 103, 104, 105],
        ];
        let s = (0, 0);
        let e = (2, 5);
        let expected = (s, e, v);
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: ((isize, isize), (isize, isize), Vec<Vec<u8>>) = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 31);
    }

    #[test]
    fn test_solve_part2() {
        let example: ((isize, isize), (isize, isize), Vec<Vec<u8>>) = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 29);
    }
}