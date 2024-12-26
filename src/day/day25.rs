use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::Position;
use itertools::Itertools;

type PreparedInput = Vec<Grid<bool>>;

fn prepare(input: &str) -> PreparedInput {
    input
        .split("\n\n")
        .map(|section| {
            Grid::from_rows(
                section
                    .lines()
                    .map(|row| row.as_bytes().iter().map(|&c| c == b'#')),
            )
        })
        .collect()
}

fn solve_part1(input: &PreparedInput) -> usize {
    let (locks, keys) = input
        .iter()
        .partition::<Vec<_>, _>(|grid| grid.contains(&Position(0, 0)));

    let locks = locks
        .into_iter()
        .map(|grid| {
            grid.columns()
                .map(|column| column.skip(1).position(|cell| !cell).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let keys = keys
        .into_iter()
        .map(|grid| {
            grid.columns()
                .map(|column| column.skip(1).position(|cell| cell).unwrap())
                .collect_vec()
        })
        .collect_vec();

    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| lock.iter().zip(key).all(|(lock, key)| *lock <= *key))
        .count()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input)),
        ctx.measure("part2", || ()),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 5);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 3);
    }
}
