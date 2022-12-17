use std::collections::{BinaryHeap, VecDeque};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day7, part1)]
pub fn solve_part1(raw_input: &str) -> usize {
    let commands: Vec<&str> = raw_input.split("$ ").collect_vec()[1..].to_vec();
    let mut stack: VecDeque<(String, usize)> = VecDeque::new();
    let mut tot: usize = 0;
    for c in commands {
        if c.starts_with("cd") {
            let dir = c.replace("cd ", "").replace("\n", "");
            if dir == ".." {
                let (_, size) = stack.pop_back().unwrap();
                if size <= 100000 {
                    tot += size;
                }
                let (parent, parent_size) = stack.pop_back()
                    .unwrap();
                stack.push_back((parent, parent_size + size));
            } else {
                stack.push_back((dir, 0));
            }
        } else {
            let (dir, mut size) = stack.pop_back().unwrap();
            c.split("\n").collect_vec()[1..].into_iter().for_each(|s| {
                let maybe_file_size = s.split(" ")
                    .collect_vec()
                    .first()
                    .unwrap()
                    .parse::<usize>();
                if maybe_file_size.is_ok() {
                    size += maybe_file_size.unwrap();
                }
            });
            stack.push_back((dir, size));
        }
    }
    while stack.len() > 1 {
        let (_, size) = stack.pop_back().unwrap();
        if size <= 100000 {
            tot += size;
        }
        let (parent, parent_size) = stack.pop_back()
            .unwrap();
        stack.push_back((parent, parent_size + size));
    }
    tot
}

#[aoc(day7, part2)]
pub fn solve_part2(raw_input: &str) -> usize {
    let commands: Vec<&str> = raw_input.split("$ ").collect_vec()[1..].to_vec();
    let mut stack: VecDeque<(String, usize)> = VecDeque::new();
    let mut dir_sizes: BinaryHeap<usize> = BinaryHeap::new();
    for c in commands {
        if c.starts_with("cd") {
            let dir = c.replace("cd ", "").replace("\n", "");
            if dir == ".." {
                let (_, size) = stack.pop_back().unwrap();
                dir_sizes.push(size);
                let (parent, parent_size) = stack.pop_back()
                    .unwrap();
                stack.push_back((parent, parent_size + size));
            } else {
                stack.push_back((dir, 0));
            }
        } else {
            let (dir, mut size) = stack.pop_back().unwrap();
            c.split("\n").collect_vec()[1..].into_iter().for_each(|s| {
                let maybe_file_size = s.split(" ")
                    .collect_vec()
                    .first()
                    .unwrap()
                    .parse::<usize>();
                if maybe_file_size.is_ok() {
                    size += maybe_file_size.unwrap();
                }
            });
            stack.push_back((dir, size));
        }
    }
    while stack.len() > 1 {
        let (_, size) = stack.pop_back().unwrap();
        dir_sizes.push(size);
        let (parent, parent_size) = stack.pop_back()
            .unwrap();
        stack.push_back((parent, parent_size + size));
    }
    let cur_tot_size = stack.pop_back().unwrap().1;
    let size_to_free: usize = 30000000 + cur_tot_size - 70000000;
    let mut ret = dir_sizes.pop().unwrap();
    while dir_sizes.len() > 1 && &size_to_free < dir_sizes.peek().unwrap() {
        ret = dir_sizes.pop().unwrap();
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&EXAMPLE), 95437);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&EXAMPLE), 24933642);
    }
}