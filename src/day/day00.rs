use crate::solution::Solution;
use crate::util::measure::MeasureContext;
use winnow::{PResult, Parser};

type PreparedInput = Vec<i64>;

fn line(input: &mut &str) -> PResult<i64, winnow::error::ContextError> {
    winnow::ascii::dec_int.parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| line.parse(l).unwrap()).collect()
}

fn solve_part1(input: &PreparedInput) -> usize {
    input.iter().count()
}

fn solve_part2(input: &PreparedInput) -> usize {
    input.iter().count()
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

    const EXAMPLE_INPUT: &str = "";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 0);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 0);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 0);
    }
}
