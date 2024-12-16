use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{Direction, Position, RotationalDirection};
use crate::util::solver::{solve_depth_first, solve_priority};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::ops::ControlFlow::{Break, Continue};

type PreparedInput = Grid<bool>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(
        input
            .lines()
            .map(|row| row.as_bytes().iter().map(|&c| c == b'#')),
    )
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
    score: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_both(input: &PreparedInput) -> (usize, usize) {
    let mut best_score = usize::MAX;

    let start_position = Position(input.dimensions.0 - 2, 1);
    let end_position = Position(1, input.dimensions.1 - 2);

    let mut best_score_map = FxHashMap::default();
    let mut add_best_score = |position: Position, direction: Direction, score: usize| -> bool {
        match best_score_map.entry((position, direction)) {
            Entry::Occupied(mut entry) => {
                if *entry.get() > score {
                    *entry.get_mut() = score;
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(score);
                true
            }
        }
    };

    solve_priority(
        |stack, s| {
            if s.score > best_score {
                return Break(());
            }
            if s.position == end_position {
                best_score = s.score;
                return Continue(());
            }

            add_best_score(s.position, s.direction, s.score);

            for (i, position) in s
                .position
                .positions(&input.dimensions, &s.direction)
                .take_while(|next_position| !input.contains(next_position))
                .enumerate()
            {
                let position_score = s.score + i + 1;
                if position == end_position {
                    best_score = position_score;
                    return Continue(());
                }
                add_best_score(position, s.direction, position_score);

                let next_rotation = s.direction.rotated(&RotationalDirection::Anticlockwise);
                let next_position = position.moved(&next_rotation);
                if !input.contains(&next_position)
                    && add_best_score(next_position, next_rotation, position_score + 1000 + 1)
                {
                    add_best_score(position, next_rotation, position_score + 1000);
                    stack.push(State {
                        position: next_position,
                        direction: next_rotation,
                        score: s.score + 1000 + i + 2,
                    });
                }
                let next_rotation = s.direction.rotated(&RotationalDirection::Clockwise);
                let next_position = position.moved(&next_rotation);
                if !input.contains(&next_position)
                    && add_best_score(next_position, next_rotation, position_score + 1000 + 1)
                {
                    add_best_score(position, next_rotation, position_score + 1000);
                    stack.push(State {
                        position: next_position,
                        direction: next_rotation,
                        score: s.score + 1000 + i + 2,
                    });
                }
            }

            Continue(())
        },
        vec![
            State {
                position: start_position,
                direction: Direction::Right,
                score: 0,
            },
            State {
                position: start_position,
                direction: Direction::Up,
                score: 1000,
            },
        ],
    )
    .unwrap();

    let mut best_visited: FxHashSet<Position> = FxHashSet::default();
    solve_depth_first(
        |stack, s| {
            best_visited.insert(s.position);
            {
                let next_position = s.position.moved(&s.direction.inverted());
                if best_score_map
                    .get(&(next_position, s.direction))
                    .map(|score| *score + 1)
                    == Some(s.score)
                {
                    stack.push(State {
                        position: next_position,
                        direction: s.direction,
                        score: s.score - 1,
                    });
                }
            }
            {
                let next_rotation = s.direction.rotated(&RotationalDirection::Anticlockwise);
                if best_score_map
                    .get(&(s.position, next_rotation))
                    .map(|score| *score + 1000)
                    == Some(s.score)
                {
                    stack.push(State {
                        position: s.position,
                        direction: next_rotation,
                        score: s.score - 1000,
                    });
                }
            }
            {
                let next_rotation = s.direction.rotated(&RotationalDirection::Clockwise);
                if best_score_map
                    .get(&(s.position, next_rotation))
                    .map(|score| *score + 1000)
                    == Some(s.score)
                {
                    stack.push(State {
                        position: s.position,
                        direction: next_rotation,
                        score: s.score - 1000,
                    });
                }
            }
        },
        vec![
            State {
                position: end_position,
                direction: Direction::Right,
                score: best_score,
            },
            State {
                position: end_position,
                direction: Direction::Up,
                score: best_score,
            },
        ],
    );

    (best_score, best_visited.len())
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SECOND_EXAMPLE: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (15, 15).into());
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 7036);
    }
    #[test]
    fn part1_second_example() {
        assert_eq!(solve_both(&prepare(SECOND_EXAMPLE)).0, 11048);
    }
    #[test]
    fn part2_example() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 45);
    }
    #[test]
    fn part2_second_example() {
        assert_eq!(solve_both(&prepare(SECOND_EXAMPLE)).1, 64);
    }
}
