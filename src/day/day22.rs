use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use rustc_hash::{FxHashMap, FxHashSet};

type PreparedInput = Vec<u64>;

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn evolve(mut number: u64) -> u64 {
    number ^= number * 64;
    number %= 16777216;

    number ^= number / 32;
    number %= 16777216;

    number ^= number * 2048;
    number %= 16777216;

    number
}

fn evolve_iter(mut number: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        number = evolve(number);
        Some(number)
    })
}

fn solve_part1(input: &PreparedInput) -> u64 {
    input
        .iter()
        .map(|num| evolve_iter(*num).nth(2000 - 1).unwrap())
        .sum()
}

fn price(number: u64) -> u64 {
    number % 10
}

fn solve_part2(input: &PreparedInput) -> u64 {
    let mut map = FxHashMap::default();

    input.iter().for_each(|number| {
        let mut found = FxHashSet::default();
        evolve_iter(*number)
            .take(2000)
            .map(price)
            .map_windows(|window: &[u64; 2]| window[1] as i64 - window[0] as i64)
            .map_windows(|window: &[i64; 4]| *window)
            .zip(evolve_iter(*number).map(price).skip(4))
            .for_each(|(window, price)| {
                if !found.insert(window) {
                    return;
                }
                *map.entry(window).or_insert(0) += price;
            })
    });

    map.into_values().max().unwrap()
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

    const EXAMPLE_PART1: &str = "1
10
100
2024";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_PART1).len(), 4);
    }
    #[test]
    fn evolve_123() {
        assert_eq!(evolve_iter(123).take(10).collect::<Vec<_>>(), [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ]);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_PART1)), 37327623);
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&prepare(
                "1
2
3
2024"
            )),
            23
        );
    }
}
