use crate::solution::Solution;
use crate::util::measure::MeasureContext;

type PreparedInput = Vec<Vec<u8>>;

fn line(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(line).collect()
}

fn is_increasing(slice: &[u8]) -> bool {
    slice
        .windows(2)
        .all(|window| window[0] < window[1] && window[1] - window[0] <= 3)
}

fn is_decreasing(slice: &[u8]) -> bool {
    slice
        .windows(2)
        .all(|window| window[0] > window[1] && window[0] - window[1] <= 3)
}

fn solve_part1(input: &PreparedInput) -> usize {
    input
        .iter()
        .filter(|report| is_increasing(report) || is_decreasing(report))
        .count()
}

fn solve_part2(input: &PreparedInput) -> usize {
    input
        .iter()
        .filter(|&report| {
            is_increasing(report)
                || is_decreasing(report)
                || (0..report.len()).any(|i| {
                    let mut report = report.clone();
                    report.remove(i);

                    is_increasing(&report) || is_decreasing(&report)
                })
        })
        .count()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> (Solution, Solution) {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input)).into(),
        ctx.measure("part2", || solve_part2(&input)).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 2);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 4);
    }
}
