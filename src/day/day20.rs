use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{DIRECTIONS, Position};
use crate::util::solver::solve_breadth_first;
use std::ops::ControlFlow::{Break, Continue};

fn parse(input: &str) -> (Grid<bool>, Position, Position) {
    let mut start = None;
    let mut end = None;
    let grid = Grid::from_rows(input.lines().enumerate().map(|(y, line)| {
        let start = &mut start;
        let end = &mut end;
        line.as_bytes()
            .iter()
            .enumerate()
            .map(move |(x, c)| match c {
                b'.' => false,
                b'#' => true,
                b'S' => {
                    *start = Some(Position::from_yx(y, x));
                    false
                }
                b'E' => {
                    *end = Some(Position::from_yx(y, x));
                    false
                }
                _ => panic!("Unexpected character"),
            })
            .collect::<Vec<_>>()
    }));

    (grid, start.unwrap(), end.unwrap())
}

type PreparedInput = Grid<usize>;

fn prepare(grid: Grid<bool>, start: Position, end: Position) -> PreparedInput {
    let mut distances = Grid::from_dimensions(grid.dimensions, usize::MAX);
    distances.set(&start, 0);
    solve_breadth_first(
        |stack, pos, time| {
            for direction in &DIRECTIONS {
                let next_position = pos.moved(direction);
                if grid.contains(&next_position) {
                    continue;
                }
                let current_distance = distances.get_mut(&next_position);
                if *current_distance <= time + 1 {
                    continue;
                }
                *current_distance = time + 1;
                if next_position == end {
                    return Break(());
                }
                stack.push(next_position);
            }
            Continue(())
        },
        vec![start],
    );

    let best_time = *distances.get(&end);

    solve_breadth_first(
        |stack, pos, time| {
            for direction in &DIRECTIONS {
                let next_position = pos.moved(direction);
                if grid.contains(&next_position) {
                    continue;
                }
                let current_distance = distances.get_mut(&next_position);
                if *current_distance <= best_time - (time + 1) {
                    continue;
                }
                *current_distance = best_time - (time + 1);
                if next_position == end {
                    return Break(());
                }
                stack.push(next_position);
            }
            Continue(())
        },
        vec![end],
    );
    distances
}

fn solve_part1(distances: &PreparedInput, minimum: usize) -> usize {
    distances
        .iter_windows3_where(|tile| *tile == usize::MAX)
        .filter(|window| {
            let left = *window.left();
            let right = *window.right();
            let top = *window.top();
            let bottom = *window.bottom();
            (left != usize::MAX
                && right != usize::MAX
                && left.abs_diff(right).saturating_sub(2) >= minimum)
                || (top != usize::MAX
                    && bottom != usize::MAX
                    && top.abs_diff(bottom).saturating_sub(2) >= minimum)
        })
        .count()
}

fn solve_part2(distances: &PreparedInput, minimum: usize) -> usize {
    const CHEAT: usize = 20;
    distances
        .positions_where(|tile| *tile != usize::MAX)
        .map(|pos| {
            let start = *distances.get(&pos);

            let mut sum = 0;
            for y in pos.0.saturating_sub(CHEAT)..(pos.0 + CHEAT + 1).min(distances.dimensions.0) {
                let y_dist = y.abs_diff(pos.0);
                let remaining_in_cheat = CHEAT - y_dist;
                for x in pos.1.saturating_sub(remaining_in_cheat)
                    ..(pos.1 + remaining_in_cheat + 1).min(distances.dimensions.1)
                {
                    let target = *distances.get(&Position::from_yx(y, x));
                    if target == usize::MAX {
                        continue;
                    }

                    let x_dist = x.abs_diff(pos.1);
                    let cheat_distance = y_dist + x_dist;
                    assert!(cheat_distance <= CHEAT);

                    let saved = target.saturating_sub(start + cheat_distance);
                    if saved >= minimum {
                        sum += 1;
                    }
                }
            }
            sum
        })
        .sum()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let (grid, start, end) = ctx.measure("parse", || parse(input));
    let input = ctx.measure("prepare", || prepare(grid, start, end));
    (
        ctx.measure("part1", || solve_part1(&input, 100)),
        ctx.measure("part2", || solve_part2(&input, 100)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn prepare_example() {
        let input = parse(EXAMPLE_INPUT);
        assert_eq!(input.0.dimensions, (15, 15).into());
    }
    #[test]
    fn part1_example() {
        let parsed = parse(EXAMPLE_INPUT);
        let prepared = prepare(parsed.0, parsed.1, parsed.2);
        assert_eq!(solve_part1(&prepared, 1), 44);
    }
    #[test]
    fn part2_example() {
        let parsed = parse(EXAMPLE_INPUT);
        let prepared = prepare(parsed.0, parsed.1, parsed.2);
        assert_eq!(solve_part2(&prepared, 70), 41);
    }
}
