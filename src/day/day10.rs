use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::Direction;
use rustc_hash::{FxHashMap, FxHashSet};

type PreparedInput = Grid<u8>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(
        input
            .lines()
            .map(|line| line.as_bytes().iter().map(|b| *b - b'0')),
    )
}

fn solve_part1(input: &PreparedInput) -> usize {
    let mut set = input
        .positions_where(|&num| num == 9)
        .map(|pos| (pos, FxHashSet::from_iter([pos])))
        .collect::<FxHashMap<_, _>>();

    for i in (0..9).rev() {
        set = input
            .positions_where(|&num| num == i)
            .map(|pos| {
                (
                    pos,
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .iter()
                    .filter_map(|direction| {
                        pos.checked_moved(&input.dimensions, direction)
                            .and_then(|above| set.get(&above).cloned())
                    })
                    .flatten()
                    .collect::<FxHashSet<_>>(),
                )
            })
            .collect::<FxHashMap<_, _>>();
    }

    set.values().map(|set| set.len()).sum()
}

fn solve_part2(input: &PreparedInput) -> usize {
    let mut set = input
        .positions_where(|&num| num == 9)
        .map(|pos| (pos, 1))
        .collect::<FxHashMap<_, _>>();

    for i in (0..9).rev() {
        set = input
            .positions_where(|&num| num == i)
            .map(|pos| {
                (
                    pos,
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .iter()
                    .filter_map(|direction| {
                        pos.checked_moved(&input.dimensions, direction)
                            .and_then(|above| set.get(&above))
                    })
                    .sum(),
                )
            })
            .collect::<FxHashMap<_, _>>();
    }

    set.values().sum()
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

    const EXAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (8, 8).into());
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 36);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 81);
    }
}
