use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use num::integer::div_rem;
use winnow::ascii::dec_uint;
use winnow::combinator::{preceded, separated_pair};
use winnow::{PResult, Parser};

#[derive(Debug, Copy, Clone)]
struct ButtonBehaviour {
    x: u8,
    y: u8,
}

#[derive(Debug, Copy, Clone)]
struct Prize {
    x: u64,
    y: u64,
}

#[derive(Debug, Copy, Clone)]
struct Machine {
    a: ButtonBehaviour,
    b: ButtonBehaviour,
    prize: Prize,
}

type PreparedInput = Vec<Machine>;

fn prepare(input: &str) -> PreparedInput {
    fn button_behaviour(input: &mut &str) -> PResult<ButtonBehaviour> {
        separated_pair(preceded("X+", dec_uint), ", ", preceded("Y+", dec_uint))
            .map(|(x, y)| ButtonBehaviour { x, y })
            .parse_next(input)
    }

    fn prize(input: &mut &str) -> PResult<Prize> {
        separated_pair(preceded("X=", dec_uint), ", ", preceded("Y=", dec_uint))
            .map(|(x, y)| Prize { x, y })
            .parse_next(input)
    }

    fn machine(input: &mut &str) -> PResult<Machine> {
        (
            preceded("Button A: ", button_behaviour),
            preceded("\nButton B: ", button_behaviour),
            preceded("\nPrize: ", prize),
        )
            .map(|(a, b, prize)| Machine { a, b, prize })
            .parse_next(input)
    }

    input
        .split("\n\n")
        .map(|l| machine.parse(l).unwrap())
        .collect()
}

fn cost(machine: &Machine) -> Option<i64> {
    let (b_presses, remainder) = div_rem(
        machine.prize.y as i64 * machine.a.x as i64 - machine.prize.x as i64 * machine.a.y as i64,
        machine.b.y as i64 * machine.a.x as i64 - machine.b.x as i64 * machine.a.y as i64,
    );
    if b_presses < 0 || remainder != 0 {
        return None;
    }
    let (a_presses, remainder) = div_rem(
        machine.prize.x as i64 - b_presses * machine.b.x as i64,
        machine.a.x as i64,
    );
    if a_presses < 0 || remainder != 0 {
        return None;
    }
    Some(a_presses * 3 + b_presses)
}

fn solve_part1(input: &PreparedInput) -> i64 {
    input.iter().filter_map(cost).sum()
}

fn solve_part2(input: &PreparedInput) -> i64 {
    input
        .iter()
        .map(|machine| Machine {
            prize: Prize {
                x: machine.prize.x + 10000000000000,
                y: machine.prize.y + 10000000000000,
            },
            ..*machine
        })
        .filter_map(|machine| cost(&machine))
        .sum()
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

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 4);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 480);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 875318608908);
    }
}
