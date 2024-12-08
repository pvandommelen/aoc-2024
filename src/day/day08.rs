use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{Dimensions, Position};
use itertools::Itertools;
use num::integer::gcd;
use rustc_hash::{FxHashMap, FxHashSet};

type PreparedInput = (Dimensions, FxHashMap<u8, Vec<Position>>);

fn prepare(input: &str) -> PreparedInput {
    let grid = Grid::from_rows(input.lines().map(|l| l.as_bytes().iter().cloned()));

    (
        grid.dimensions,
        grid.iter().filter(|(_, tile)| **tile != b'.').fold(
            FxHashMap::default(),
            |mut acc, (pos, tile)| {
                acc.entry(*tile).or_default().push(pos);
                acc
            },
        ),
    )
}

fn solve_both(input: &PreparedInput) -> (usize, usize) {
    let mut p1 = FxHashSet::default();
    let mut p2 = FxHashSet::default();
    input.1.values().for_each(|positions| {
        positions.iter().combinations(2).for_each(|permutation| {
            assert_eq!(permutation.len(), 2);

            let offset = *permutation[1] - *permutation[0];
            let gcd = gcd(offset.0, offset.1);

            let offset = offset / gcd;
            p2.insert(*permutation[0]);
            permutation[0]
                .positions_steps(&input.0, &offset)
                .for_each(|pos| {
                    let a = permutation[0].manhattan_distance(&pos);
                    let b = permutation[1].manhattan_distance(&pos);
                    if a == b * 2 || a * 2 == b {
                        p1.insert(pos);
                    }
                    p2.insert(pos);
                });
            permutation[0]
                .positions_steps(&input.0, &-offset)
                .for_each(|pos| {
                    let a = permutation[0].manhattan_distance(&pos);
                    let b = permutation[1].manhattan_distance(&pos);
                    if a == b * 2 || a * 2 == b {
                        p1.insert(pos);
                    }
                    p2.insert(pos);
                });
        })
    });

    (p1.len(), p2.len())
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).0, Dimensions(12, 12));
        assert_eq!(prepare(EXAMPLE_INPUT).1.len(), 2);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 14);
    }
    #[test]
    fn example_part1_2() {
        assert_eq!(
            solve_both(&prepare(
                "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            ))
            .0,
            2
        );
    }
    #[test]
    fn example_part1_3() {
        assert_eq!(
            solve_both(&prepare(
                "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
            ))
            .0,
            4
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 34);
    }
    #[test]
    fn example_part2_2() {
        assert_eq!(
            solve_both(&prepare(
                "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
            ))
            .1,
            9
        );
    }
}
