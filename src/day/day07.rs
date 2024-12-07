use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use itertools::Itertools;
use std::iter::repeat_n;
use winnow::ascii::dec_uint;
use winnow::combinator::{separated, separated_pair};
use winnow::{PResult, Parser};

type Equation = (u64, Vec<u64>);
type PreparedInput = Vec<Equation>;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Mul,
    Add,
    Concat,
}

fn line(input: &mut &str) -> PResult<Equation, winnow::error::ContextError> {
    separated_pair(dec_uint, ": ", separated(2.., dec_uint::<_, u64, _>, ' ')).parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| line.parse(l).unwrap()).collect()
}

fn calc(numbers: &[u64], operators: &[Operator]) -> u64 {
    assert_eq!(numbers.len(), operators.len() + 1);
    numbers
        .iter()
        .skip(1)
        .zip(operators)
        .fold(numbers[0], |current, (num, op)| match op {
            Operator::Mul => current * num,
            Operator::Add => current + num,
            Operator::Concat => current * 10u64.pow(num.ilog10() + 1) + num,
        })
}

fn test<const N: usize>(
    expected_result: u64,
    numbers: &[u64],
    available_operators: [Operator; N],
) -> bool {
    repeat_n(available_operators, numbers.len() - 1)
        .multi_cartesian_product()
        .any(|c| calc(numbers, &c) == expected_result)
}

fn solve_both(input: &PreparedInput) -> (u64, u64) {
    input
        .iter()
        .filter_map(|eq| {
            let p1 = test(eq.0, &eq.1, [Operator::Mul, Operator::Add]);
            if p1 {
                return Some((eq.0, eq.0));
            }
            let p2 = test(eq.0, &eq.1, [
                Operator::Mul,
                Operator::Add,
                Operator::Concat,
            ]);
            if p2 {
                return Some((0, eq.0));
            }
            None
        })
        .reduce(|(p1, p2), (p1_entry, p2_entry)| (p1 + p1_entry, p2 + p2_entry))
        .unwrap()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 9);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 3749);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 11387);
    }
}
