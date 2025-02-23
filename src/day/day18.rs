use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{DIRECTIONS, Dimensions, Position};
use crate::util::solver::solve_breadth_first;
use std::cmp::Ordering;
use std::ops::ControlFlow;
use winnow::ascii::dec_uint;
use winnow::combinator::separated_pair;
use winnow::{ModalResult, Parser};

type PreparedInput = Vec<Position>;

fn line(input: &mut &str) -> ModalResult<Position> {
    separated_pair(dec_uint, ',', dec_uint)
        .map(|(x, y)| Position::from_yx(y, x))
        .parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    input
        .lines()
        .map(|l| line.parse(l).unwrap_or_else(|e| panic!("{}", e)))
        .collect()
}

fn attempt(mut grid: Grid<bool>) -> Option<usize> {
    let end_position = Position(grid.dimensions.0 - 1, grid.dimensions.1 - 1);

    solve_breadth_first(
        |stack, position, _| {
            if *position == end_position {
                return ControlFlow::Break(());
            }

            for direction in &DIRECTIONS {
                let Some(next_position) = position.checked_moved(&grid.dimensions, direction)
                else {
                    continue;
                };
                let available = grid.insert(&next_position);
                if !available {
                    continue;
                }
                stack.push(next_position);
            }

            ControlFlow::Continue(())
        },
        vec![Position(0, 0)],
    )
    .map(|(_, steps)| steps)
}

fn solve_part1(input: &PreparedInput, dimensions: Dimensions, limit: usize) -> usize {
    let grid = Grid::<bool>::from_positions(dimensions, input.iter().take(limit).cloned());
    attempt(grid).unwrap()
}

fn solve_part2(input: &PreparedInput, dimensions: Dimensions, skip: usize) -> String {
    let grid = Grid::<bool>::from_positions(dimensions, input[0..skip].iter().cloned());

    let indices = (skip..input.len()).collect::<Vec<_>>();
    let found = indices.binary_search_by(|i| {
        let mut grid = grid.clone();
        grid.extend(input[skip..*i].iter().copied());
        match attempt(grid) {
            None => Ordering::Greater,
            Some(_) => Ordering::Less,
        }
    });
    match found {
        Ok(_) => unreachable!(),
        Err(offset) => {
            let byte = input[skip + offset - 1];
            format!("{},{}", byte.1, byte.0)
        }
    }
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input, Dimensions(71, 71), 1024)),
        ctx.measure("part2", || solve_part2(&input, Dimensions(71, 71), 1024)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 25);
    }
    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&prepare(EXAMPLE_INPUT), Dimensions(7, 7), 12),
            22
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&prepare(EXAMPLE_INPUT), Dimensions(7, 7), 12),
            "6,1"
        );
    }
}
