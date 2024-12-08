use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::intset::IntSet;
use crate::util::measure::MeasureContext;
use crate::util::position::{Direction, Position, RotationalDirection};
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Obstruction,
    GuardUpwardFacing,
}

type PreparedInput = Grid<Tile>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstruction,
            '^' => Tile::GuardUpwardFacing,
            _ => panic!("Unexpected char {}", c),
        })
    }))
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum WalkResult {
    OutOfBounds,
    Loop,
}

type VisitedWithPositionSet = IntSet;

fn walk(
    grid: &PreparedInput,
    mut visited: VisitedWithPositionSet,
    pos: &Position,
    direction: &Direction,
    obstruction_test_fn: impl Fn(&Position) -> bool,
    mut each_fn: impl FnMut(&Position, &Direction, &VisitedWithPositionSet),
) -> WalkResult {
    assert!(grid.dimensions.0 < 256 && grid.dimensions.1 < 256);

    let mut pos = *pos;
    let mut direction = *direction;

    'next_direction: loop {
        each_fn(&pos, &direction, &visited);
        for next_pos in pos.positions(&grid.dimensions, &direction.clone()) {
            let next = grid.get(&next_pos);

            if matches!(next, Tile::Obstruction) || obstruction_test_fn(&next_pos) {
                let pos_and_direction: u32 =
                    ((pos.0 as u32) << 10) | ((pos.1 as u32) << 2) | direction as u32;
                let previous_visit = visited.insert(pos_and_direction as usize);
                if !previous_visit {
                    return WalkResult::Loop;
                }

                direction = direction.rotated(&RotationalDirection::Clockwise);
                continue 'next_direction;
            }
            each_fn(&next_pos, &direction, &visited);
            pos = next_pos;
        }
        return WalkResult::OutOfBounds;
    }
}

fn solve_both(input: &PreparedInput) -> (usize, usize) {
    let mut visited = FxHashSet::default();

    let pos = input
        .positions_where(|tile| *tile == Tile::GuardUpwardFacing)
        .next()
        .unwrap();

    let mut extra_obstructions = 0;

    let result = walk(
        input,
        Default::default(),
        &pos,
        &Direction::Up,
        |_| false,
        |pos, direction, visited_with_direction| {
            visited.insert(*pos);

            let Some(obstruction_pos) = pos.checked_moved(&input.dimensions, direction) else {
                return;
            };
            let obstruction_tile = input.get(&obstruction_pos);

            if matches!(obstruction_tile, Tile::Empty) && !visited.contains(&obstruction_pos) {
                let rotated_direction = direction.rotated(&RotationalDirection::Clockwise);
                if matches!(
                    walk(
                        input,
                        visited_with_direction.clone(),
                        pos,
                        &rotated_direction,
                        |pos| obstruction_pos == *pos,
                        |_, _, _| {}
                    ),
                    WalkResult::Loop
                ) {
                    extra_obstructions += 1;
                }
            }
        },
    );
    assert_eq!(result, WalkResult::OutOfBounds);

    (visited.len(), extra_obstructions)
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (10, 10).into());
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 41);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 6);
    }

    #[rstest]
    #[case(".#.\n#^#\n...", 1)]
    #[case("...\n#^#\n.#.", 1)]
    #[case("...\n..#\n#^.\n.#.", 1)]
    fn part2_extra(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_both(&prepare(input)).1, expected);
    }
}
