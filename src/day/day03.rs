use crate::solution::Solution;
use crate::util::measure::MeasureContext;
use regex::Regex;
use std::sync::LazyLock;

fn solve_part1(input: &str) -> u32 {
    static REGEX: LazyLock<Regex, fn() -> Regex> =
        LazyLock::new(|| Regex::new(r"mul\(((?-u:\d+)),((?-u:\d+))\)").unwrap());

    REGEX
        .captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    static REGEX: LazyLock<Regex, fn() -> Regex> = LazyLock::new(|| {
        Regex::new(r"(?:do\(\))|(?:don\'t\(\))|(?:mul\(((?-u:\d+)),((?-u:\d+))\))").unwrap()
    });

    let mut enabled = true;
    REGEX
        .captures_iter(input)
        .map(|c| match &c[0] {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if enabled {
                    c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap()
                } else {
                    0
                }
            }
        })
        .sum()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> (Solution, Solution) {
    (
        ctx.measure("part1", || solve_part1(&input)).into(),
        ctx.measure("part2", || solve_part2(&input)).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(
            solve_part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solve_part2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
