use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use rustc_hash::{FxBuildHasher, FxHashMap};

type PreparedInput = Vec<u64>;

fn prepare(input: &str) -> PreparedInput {
    input
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink_iterations(input: &PreparedInput, n: u32) -> u64 {
    let mut stones = input
        .iter()
        .map(|stone| (*stone, 1u64))
        .collect::<FxHashMap<_, _>>();

    for _ in 0..n {
        let mut next = FxHashMap::with_capacity_and_hasher(stones.len() * 2, FxBuildHasher);
        stones.into_iter().for_each(|(stone, count)| {
            if stone == 0 {
                *next.entry(1).or_insert(0) += count;
            } else {
                let width = stone.ilog10() + 1;
                if width % 2 == 0 {
                    let factor = 10u64.pow(width / 2);
                    *next.entry(stone / factor).or_insert(0) += count;
                    *next.entry(stone % factor).or_insert(0) += count;
                } else {
                    *next.entry(stone * 2024).or_insert(0) += count;
                }
            }
        });
        stones = next;
    }

    stones.values().sum()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || blink_iterations(&input, 25)),
        ctx.measure("part2", || blink_iterations(&input, 75)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 2);
    }
    #[test]
    fn example_part1_short() {
        assert_eq!(blink_iterations(&prepare("0 1 10 99 999"), 1), 7);
    }
    #[test]
    fn example_part1() {
        assert_eq!(blink_iterations(&prepare(EXAMPLE_INPUT), 25), 55312);
    }
}
