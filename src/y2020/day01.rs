use itertools::Itertools;

/// Solves the 2020 day 1 problem. Function chaining seems to be pretty
/// well-supported?
pub fn part_a(input: &str) -> Vec<u32> {
    solve(input, 2)
}

pub fn part_b(input: &str) -> Vec<u32> {
    solve(input, 3)
}

fn solve(input: &str, n: usize) -> Vec<u32> {
    return input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .combinations(n)
        .filter(|n| n.iter().sum::<u32>() == 2020)
        .map(|n| n.iter().product())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example puzzle input
    static SAMPLE: &'static [u32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part_a_solution() {
        let input = SAMPLE.iter().map(|n| n.to_string()).join("\n");
        let solution = part_a(&input);
        assert_eq!(solution, [514579]);
    }

    #[test]
    fn part_b_solution() {
        let input = SAMPLE.iter().map(|n| n.to_string()).join("\n");
        let solution = part_b(&input);
        assert_eq!(solution, [241861950]);
    }
}
