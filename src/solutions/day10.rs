use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::mod_floor;
use crate::solutions::day10::Op::{AddX, NoOp};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Op {
    NoOp,
    AddX(isize)
}

#[aoc_generator(day10)]
pub fn generator(raw_input: &str) -> Vec<Op> {
    raw_input.split("\n")
        .into_iter()
        .map(|line| {
            let split_line = line.split(" ").collect_vec();
            let op_type = split_line[0];
            match op_type {
                "noop" => NoOp,
                "addx" => {
                    AddX(split_line[1].parse().unwrap())
                }
                _ => panic!("Invalid op type: {}", op_type)
            }
        }).collect_vec()
}

fn perform_op(op: &Op, current_cycle: isize, current_x: isize) -> (isize, isize) {
    match op {
        NoOp => (current_cycle + 1, current_x),
        AddX(v) => (current_cycle + 2, current_x + v)
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(ops: &[Op]) -> isize {
    let mut current_cycle = 1;
    let mut current_reg = 1;
    let pois: Vec<isize> = vec![
        20,
        60,
        100,
        140,
        180,
        220
    ];
    let mut poinum = 0;
    let mut signal_strength_sum = 0;
    for op in ops {
        let prev_reg = current_reg;
        (current_cycle, current_reg) = perform_op(op, current_cycle, current_reg);
        if current_cycle > pois[poinum] {
            signal_strength_sum += pois[poinum] * prev_reg;
            poinum += 1;
        }
        if poinum >= pois.len() {
            break;
        }
    }
    signal_strength_sum
}

#[aoc(day10, part2)]
pub fn solve_part2(ops: &[Op]) -> String {
    let mut crt: [[char; 40]; 6] = [['.'; 40];6];
    let mut current_cycle = 0;
    let mut current_reg = 1;
    for op in ops {
        let prev_reg = current_reg;
        let prev_cycle = current_cycle;
        (current_cycle, current_reg) = perform_op(op, current_cycle, current_reg);
        for i in prev_cycle..current_cycle {
            let row_pos = mod_floor(i, 40);
            if (row_pos -1 <= prev_reg) & (row_pos +1 >= prev_reg) {
                crt[(i/40) as usize][row_pos as usize] = '#';
            }
        }

    }
    let readout = crt.iter().map(|row| { row.iter().join("") }).join("\n");
    format!("\n{}", readout)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "noop
addx 3
addx -5";

    const EXAMPLE2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const PART2_SOLN: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn test_generator() {
        let expected = vec![
            NoOp,
            AddX(3),
            AddX(-5)
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<Op> = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 13140);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Op> = generator(&EXAMPLE2);
        assert_eq!(solve_part2(&example), PART2_SOLN);
    }
}