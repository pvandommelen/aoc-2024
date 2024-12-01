use crate::solution::Solution;
use rustc_hash::FxHashMap;
use winnow::ascii::multispace1;
use winnow::combinator::separated_pair;
use winnow::{PResult, Parser};

type PreparedInput = (Vec<u32>, Vec<u32>);

fn line(input: &mut &str) -> PResult<(u32, u32), winnow::error::ContextError> {
    separated_pair(
        winnow::ascii::dec_uint,
        multispace1,
        winnow::ascii::dec_uint,
    )
    .parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| line.parse(l).unwrap()).unzip()
}

fn solve_part1(input: &PreparedInput) -> u32 {
    let (mut a, mut b) = input.clone();
    a.sort_unstable();
    b.sort_unstable();

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn solve_part2(input: &PreparedInput) -> u32 {
    let (a, b) = input;

    let mut counts = FxHashMap::default();
    for num in b {
        *counts.entry(*num).or_default() += 1;
    }

    a.iter()
        .map(|num| {
            let occurrences = counts.get(num).unwrap_or(&0);
            num * occurrences
        })
        .sum()
}

pub fn solve(input: &str) -> (Solution, Solution) {
    let input = prepare(input);
    (solve_part1(&input).into(), solve_part2(&input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).0.len(), 6);
        assert_eq!(prepare(EXAMPLE_INPUT).1.len(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 11);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 31);
    }
}
