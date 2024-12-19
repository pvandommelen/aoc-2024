#![allow(clippy::needless_range_loop)]

use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Towel(Vec<u8>);
impl From<&str> for Towel {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct PartialTowel<'a>(&'a [u8]);
impl<'a> From<&'a str> for PartialTowel<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.as_bytes())
    }
}
impl<'a> From<&'a Towel> for PartialTowel<'a> {
    fn from(towel: &'a Towel) -> Self {
        PartialTowel(towel.0.as_slice())
    }
}

#[derive(Default)]
struct TowelSet<'a> {
    w: FxHashSet<&'a [u8]>,
    u: FxHashSet<&'a [u8]>,
    b: FxHashSet<&'a [u8]>,
    r: FxHashSet<&'a [u8]>,
    g: FxHashSet<&'a [u8]>,
    largest: usize,
}
impl<'a> TowelSet<'a> {
    fn contains(&self, t: &PartialTowel) -> bool {
        match t.0[0] {
            b'w' => self.w.contains(&t.0[1..]),
            b'u' => self.u.contains(&t.0[1..]),
            b'b' => self.b.contains(&t.0[1..]),
            b'r' => self.r.contains(&t.0[1..]),
            b'g' => self.g.contains(&t.0[1..]),
            _ => unreachable!(),
        }
    }
    fn insert(&mut self, t: PartialTowel<'a>) -> bool {
        self.largest = self.largest.max(t.0.len());
        match t.0[0] {
            b'w' => self.w.insert(&t.0[1..]),
            b'u' => self.u.insert(&t.0[1..]),
            b'b' => self.b.insert(&t.0[1..]),
            b'r' => self.r.insert(&t.0[1..]),
            b'g' => self.g.insert(&t.0[1..]),
            _ => unreachable!(),
        }
    }
}
impl<'a> FromIterator<PartialTowel<'a>> for TowelSet<'a> {
    fn from_iter<I: IntoIterator<Item = PartialTowel<'a>>>(iter: I) -> Self {
        let mut set = TowelSet::default();
        for towel in iter {
            set.insert(towel);
        }
        set
    }
}

type PreparedInput<'a> = (TowelSet<'a>, Vec<Towel>);

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

fn get_number_of_combinations(available: &TowelSet, target: &Towel) -> u64 {
    let mut cache = vec![0; target.0.len()];

    for offset in (0..target.0.len()).rev() {
        let remaining_length = (target.0.len() - offset).min(available.largest);

        let mut sum = 0;
        for i in offset + 1..offset + remaining_length + 1 {
            if !available.contains(&PartialTowel(&target.0[offset..i])) {
                continue;
            }
            if i == target.0.len() {
                sum += 1;
            } else {
                sum += cache[i];
            }
        }
        cache[offset] = sum;
    }

    cache[0]
}

fn solve_both(input: &PreparedInput) -> (usize, u64) {
    let (available, target) = input;
    target
        .iter()
        .map(|target| get_number_of_combinations(available, target))
        .fold((0, 0), |(mut count, mut sum), res| {
            sum += res;
            if res > 0 {
                count += 1;
            }
            (count, sum)
        })
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
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
        assert_eq!(prepared.1.len(), 8);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 6);
    }
    #[test]
    fn part2_example() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 16);
    }
}
