use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{Dimensions, Direction, Position};
use crate::util::solver::solve_priority;
use std::cmp::Ordering;
use std::ops::ControlFlow;
use winnow::ascii::dec_uint;
use winnow::combinator::separated_pair;
use winnow::{PResult, Parser};

type PreparedInput = Vec<Position>;

fn line(input: &mut &str) -> PResult<Position> {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    end_position: Position,
    steps: usize,
}
impl State {
    fn score(&self) -> usize {
        self.steps + self.position.manhattan_distance(&self.end_position)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score()).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn attempt(input: &PreparedInput, dimensions: Dimensions, limit: usize) -> Option<State> {
    let mut grid = Grid::<bool>::from_positions(dimensions, input.iter().take(limit).cloned());

    let s = solve_priority(
        |stack, s| {
            if s.position == s.end_position {
                return ControlFlow::Break(());
            }

            for direction in &DIRECTIONS {
                let Some(next_position) = s.position.checked_moved(&grid.dimensions, direction)
                else {
                    continue;
                };
                let available = grid.insert(&next_position);
                if !available {
                    continue;
                }
                stack.push(State {
                    position: next_position,
                    end_position: s.end_position,
                    steps: s.steps + 1,
                });
            }

            ControlFlow::Continue(())
        },
        vec![State {
            position: Position(0, 0),
            end_position: Position(dimensions.0 - 1, dimensions.1 - 1),
            steps: 0,
        }],
    );

    s
}

fn solve_part1(input: &PreparedInput, dimensions: Dimensions, limit: usize) -> usize {
    attempt(input, dimensions, limit).map(|s| s.steps).unwrap()
}

fn solve_part2(input: &PreparedInput, dimensions: Dimensions, skip: usize) -> String {
    for (i, byte) in input.iter().enumerate().skip(skip) {
        if attempt(input, dimensions, i + 1).map(|s| s.steps).is_none() {
            return format!("{},{}", byte.1, byte.0);
        }
    }
    panic!("End always reachable");
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
