use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use winnow::{PResult, Parser};

type PreparedInput = Vec<i64>;

fn line(input: &mut &str) -> PResult<i64> {
    winnow::ascii::dec_int.parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    input
        .lines()
        .map(|l| line.parse(l).unwrap_or_else(|e| panic!("{}", e)))
        .collect()
}

fn solve_part1(input: &PreparedInput) -> usize {
    input.iter().count()
}

fn solve_part2(input: &PreparedInput) -> usize {
    input.iter().count()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input)),
        ctx.measure("part2", || solve_part2(&input)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 0);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 0);
    }
    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 0);
    }
}
