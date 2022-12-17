use std::cmp::max;
use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn generator(raw_input: &str) -> Vec<Vec<u8>> {
    raw_input.split("\n").map(|row| {
        row.chars()
            .map(|c| c.to_digit(10).unwrap() as u8 + 1 )
            .collect_vec()
    }).collect_vec()
}

#[aoc(day8, part1)]
pub fn solve_part1(tree_heights: &[Vec<u8>]) -> usize {
    let mut visible = HashSet::new();
    let _ = tree_heights.into_iter()
        .enumerate()
        .foreach(|(row, row_val)| {
            row_val.into_iter()
                .enumerate()
                .fold(0 as u8, |tallest, (col, &val)| {
                    if val > tallest {
                        visible.insert((row, col));
                        return val;
                    }
                    tallest
                });
            row_val.into_iter()
                .enumerate()
                .rfold(0 as u8, |tallest, (col, &val)| {
                    if val > tallest {
                        visible.insert((row, col));
                        return val;
                    }
                    tallest
                });
        });
    let _ = (0..tree_heights.first().unwrap().len()).into_iter().map(|row| {
        (0..tree_heights.len()).into_iter().map(|col| {
            tree_heights.get(col).unwrap().get(row).unwrap().clone()
        }).collect_vec()
    }).enumerate()
        .foreach(|(col, col_val)| {
            (&col_val).into_iter()
                .enumerate()
                .fold(0 as u8, |tallest, (row, &val)| {
                    if val > tallest {
                        visible.insert((row, col));
                        return val;
                    }
                    tallest
                });
            col_val.into_iter()
                .enumerate()
                .rfold(0 as u8, |tallest, (row, val)| {
                    if val > tallest {
                        visible.insert((row, col));
                        return val;
                    }
                    tallest
                });
        });
    visible.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(tree_heights: &[Vec<u8>]) -> usize {
    let mut max_score: usize = 0;
    let rows = tree_heights.len();
    let cols = tree_heights.first().unwrap().len();
    for row in 0..tree_heights.len() {
        for col in 0..tree_heights.first().unwrap().len() {
            let tree_height = tree_heights.get(row)
                .unwrap()
                .get(col)
                .unwrap();
            let mut up = col;
            for offset in 1..=col {
                let offset_tree = tree_heights.get(row)
                    .unwrap()
                    .get(col - offset)
                    .unwrap();
                if offset_tree >= tree_height {
                    up = offset;
                    break;
                }
            }
            let mut down = cols - col - 1;
            for offset in 1..(cols-col) {
                let offset_tree = tree_heights.get(row)
                    .unwrap()
                    .get(col + offset)
                    .unwrap();
                if offset_tree >= tree_height {
                    down = offset;
                    break;
                }
            }
            let mut left = row;
            for offset in 1..=row {
                let offset_tree = tree_heights.get(row - offset)
                    .unwrap()
                    .get(col)
                    .unwrap();
                if offset_tree >= tree_height {
                    left = offset;
                    break;
                }
            }
            let mut right = rows - row - 1;
            for offset in 1..(rows-row) {
                let offset_tree = tree_heights.get(row + offset)
                    .unwrap()
                    .get(col)
                    .unwrap();
                if offset_tree >= tree_height {
                    right = offset;
                    break;
                }
            }
            max_score = max(max_score, left * right * up * down)
        }
    }
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_generator() {
        let expected = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0],
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<Vec<u8>> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 21);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Vec<u8>> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 8);
    }
}