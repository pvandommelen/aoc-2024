use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Towel(Vec<u8>);
impl FromStr for Towel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Towel(s.as_bytes().to_vec()))
    }
}
impl From<&str> for Towel {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}
impl Debug for Towel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::str::from_utf8(&self.0).unwrap().fmt(f)
    }
}

type PreparedInput = (FxHashSet<Towel>, Vec<Towel>);

fn prepare(input: &str) -> PreparedInput {
    let mut sections = input.split("\n\n");
    let available_section = sections.next().unwrap();
    let target_section = sections.next().unwrap();
    assert_eq!(sections.next(), None);

    (
        available_section.split(", ").map_into().collect(),
        target_section.lines().map_into().collect(),
    )
}

fn is_possible(available: &FxHashSet<Towel>, target: &Towel, cache: &mut [bool]) -> bool {
    if !cache[0] {
        return false;
    }
    if available.contains(target) {
        return true;
    }
    for i in (1..target.0.len()).rev() {
        if available.contains(&Towel(target.0[0..i].to_vec()))
            && is_possible(available, &Towel(target.0[i..].to_owned()), &mut cache[i..])
        {
            return true;
        }
    }
    cache[0] = false;
    false
}

fn solve_part1(input: &PreparedInput) -> usize {
    let (available, target) = input;
    target
        .iter()
        .filter(|&target| {
            let mut cache = vec![true; available.len()];
            is_possible(available, target, &mut cache)
        })
        .count()
}

fn get_number_of_combinations(
    available: &FxHashSet<Towel>,
    target: &Towel,
    cache: &mut [u64],
) -> u64 {
    if cache[0] != u64::MAX {
        return cache[0];
    }
    let mut sum = 0;
    if available.contains(target) {
        sum += 1;
    }
    for i in (1..target.0.len()).rev() {
        if available.contains(&Towel(target.0[0..i].to_vec())) {
            sum += get_number_of_combinations(
                available,
                &Towel(target.0[i..].to_owned()),
                &mut cache[i..],
            );
        }
    }
    cache[0] = sum;
    sum
}

fn solve_part2(input: &PreparedInput) -> u64 {
    let (available, target) = input;
    target
        .iter()
        .map(|target| {
            let mut cache = vec![u64::MAX; available.len()];
            get_number_of_combinations(available, target, &mut cache)
        })
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

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn prepare_example() {
        let prepared = prepare(EXAMPLE_INPUT);
        assert_eq!(prepared.0.len(), 8);
        assert_eq!(prepared.1.len(), 8);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 6);
    }
    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 16);
    }
}
