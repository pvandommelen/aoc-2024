use crate::solution::Solution;
use crate::util::measure::MeasureContext;
use winnow::Parser;
use winnow::ascii::dec_uint;

fn sum_stretch(stretch: &str) -> u32 {
    stretch
        .split("mul(")
        .skip(1)
        .map(|part| {
            (dec_uint::<_, u32, ()>, ',', dec_uint::<_, u32, ()>, ')')
                .parse_next(&mut part.as_bytes())
                .map_or(0, |(a, _, b, _)| a * b)
        })
        .sum()
}

fn solve_both(mut input: &str) -> (u32, u32) {
    let mut disabled = 0;
    let mut enabled = 0;
    loop {
        let (stretch, rest) = input.split_once("don't()").unwrap_or((input, ""));
        enabled += sum_stretch(stretch);

        let Some((stretch, rest)) = rest.split_once("do()") else {
            break;
        };
        disabled += sum_stretch(stretch);
        input = rest;
    }
    (enabled + disabled, enabled)
}

pub fn solve(_ctx: &mut MeasureContext, input: &str) -> (Solution, Solution) {
    let both = solve_both(input);
    (both.0.into(), both.1.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(
            solve_both("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))").0,
            161
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solve_both("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
                .1,
            48
        );
    }
}
