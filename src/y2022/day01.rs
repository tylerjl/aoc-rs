extern crate pom;

use itertools::Itertools;
use pom::parser::*;
use std::str;

pub fn part_a(input: String) -> i32 {
    let elves = solve(input);
    return *elves.iter().max().unwrap();
}

pub fn part_b(input: String) -> i32 {
    let elves = solve(input);
    return elves.iter().sorted().rev().take(3).sum();
}

fn solve(input: String) -> Vec<i32> {
    let integer = one_of(b"0123456789")
        .repeat(1..)
        .collect()
        .convert(str::from_utf8)
        .convert(|s| s.parse::<i32>());
    let calories = integer - sym(b'\n');
    let inventory = calories.repeat(1..);
    let elves = list(inventory, sym(b'\n'));

    let result = elves.parse(input.as_bytes()).unwrap();
    let solution: Vec<i32> = result.iter().map(|x| x.iter().sum::<i32>()).collect();
    return solution;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_solution() {
        let sample = std::fs::read_to_string("resources/y2022/day01_test.txt").unwrap();
        let solution = part_a(sample);
        assert_eq!(solution, 24000);
    }

    #[test]
    fn part_b_solution() {
        let sample = std::fs::read_to_string("resources/y2022/day01_test.txt").unwrap();
        let solution = part_b(sample);
        assert_eq!(solution, 45000);
    }
}
