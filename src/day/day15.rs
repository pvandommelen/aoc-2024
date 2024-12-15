use crate::solution::SolutionTuple;
use crate::util::grid::{CellDisplay, Grid};
use crate::util::measure::MeasureContext;
use crate::util::position::{Direction, Position};
use std::fmt::{Formatter, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Box,
}
impl CellDisplay for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Box => 'O',
        })
    }
}

type PreparedInput = (Grid<Tile>, Position, Vec<Direction>);

fn prepare(input: &str) -> PreparedInput {
    let mut sections = input.split("\n\n");
    let grid_section = sections.next().unwrap();
    let movements_section = sections.next().unwrap();
    assert_eq!(sections.next(), None);

    let grid_items = grid_section.lines().map(|line| line.as_bytes());

    let grid = Grid::from_rows(grid_items.clone().map(|row| {
        row.iter().map(|c| match *c {
            b'.' | b'@' => Tile::Empty,
            b'#' => Tile::Wall,
            b'O' => Tile::Box,
            _ => panic!(),
        })
    }));

    let starting_position = grid_items
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == b'@').map(|x| Position(y, x)))
        .unwrap();

    let movements = movements_section
        .as_bytes()
        .iter()
        .filter(|&c| *c != b'\n')
        .map(|c| match *c {
            b'^' => Direction::Up,
            b'>' => Direction::Right,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    (grid, starting_position, movements)
}

fn gps(pos: &Position) -> usize {
    pos.0 * 100 + pos.1
}

fn solve_part1(input: &PreparedInput) -> usize {
    let (grid, starting_position, movements) = input;
    let mut grid = grid.clone();
    let mut position = *starting_position;
    for mov in movements {
        let mov_positions = position.positions(&grid.dimensions, mov);

        let can_move = mov_positions
            .map(|tile_pos| (tile_pos, grid.get(&tile_pos)))
            .take_while(|(_, tile)| **tile != Tile::Wall)
            .find_map(|(tile_pos, tile)| matches!(tile, Tile::Empty).then(|| tile_pos));

        let Some(boxes_move_up_to) = can_move else {
            continue;
        };
        position = position.moved(mov);
        if boxes_move_up_to != position {
            grid.set(&position, Tile::Empty);
            grid.set(&boxes_move_up_to, Tile::Box);
        }
    }

    grid.positions_where(|tile| *tile == Tile::Box)
        .map(|pos| gps(&pos))
        .sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ScaledTile {
    Empty,
    Wall,
    Box,
    RightSideBox,
}
impl CellDisplay for ScaledTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            ScaledTile::Empty => ' ',
            ScaledTile::Wall => '█',
            ScaledTile::Box => '[',
            ScaledTile::RightSideBox => ']',
        })
    }
}

fn try_move(
    grid: &Grid<ScaledTile>,
    position: &Position,
    direction: &Direction,
) -> Option<Vec<Position>> {
    let next_position = position.moved(direction);
    let tile = grid.get(&next_position);
    match tile {
        ScaledTile::Empty => Some(vec![]),
        ScaledTile::Wall => None,
        ScaledTile::Box => {
            let box_position = next_position;
            let right_side_box_position = next_position.moved(&Direction::Right);
            match direction {
                Direction::Up | Direction::Down => try_move(grid, &box_position, direction)
                    .and_then(|mut v| {
                        try_move(grid, &right_side_box_position, direction).map(|v2| {
                            v.extend(v2);
                            v.push(box_position);
                            v
                        })
                    }),
                Direction::Right => {
                    try_move(grid, &right_side_box_position, direction).map(|mut v| {
                        v.push(next_position);
                        v
                    })
                }
                Direction::Left => unreachable!(),
            }
        }
        ScaledTile::RightSideBox => {
            let box_position = next_position.moved(&Direction::Left);
            let right_side_box_position = next_position;
            match direction {
                Direction::Up | Direction::Down => try_move(grid, &box_position, direction)
                    .and_then(|mut v| {
                        try_move(grid, &right_side_box_position, direction).map(|v2| {
                            v.extend(v2);
                            v.push(box_position);
                            v
                        })
                    }),
                Direction::Right => unreachable!(),
                Direction::Left => try_move(grid, &box_position, direction).map(|mut v| {
                    v.push(box_position);
                    v
                }),
            }
        }
    }
}

fn solve_part2(input: &PreparedInput) -> usize {
    let (grid, starting_position, movements) = input;

    let mut grid = Grid::from_rows(grid.rows().map(|row| {
        row.iter().flat_map(|tile| match tile {
            Tile::Empty => [ScaledTile::Empty, ScaledTile::Empty],
            Tile::Wall => [ScaledTile::Wall, ScaledTile::Wall],
            Tile::Box => [ScaledTile::Box, ScaledTile::RightSideBox],
        })
    }));

    let mut position = Position(starting_position.0, starting_position.1 * 2);
    for mov in movements {
        let Some(moved_boxes) = try_move(&grid, &position, mov) else {
            continue;
        };

        moved_boxes.iter().for_each(|box_pos| {
            grid.set(box_pos, ScaledTile::Empty);
            grid.set(&box_pos.moved(&Direction::Right), ScaledTile::Empty);
        });
        moved_boxes.iter().for_each(|box_pos| {
            grid.set(&box_pos.moved(mov), ScaledTile::Box);
            grid.set(
                &box_pos.moved(mov).moved(&Direction::Right),
                ScaledTile::RightSideBox,
            );
        });

        position = position.moved(mov);
    }

    grid.positions_where(|tile| *tile == ScaledTile::Box)
        .map(|pos| gps(&pos))
        .sum()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let prepared_input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&prepared_input)),
        ctx.measure("part2", || solve_part2(&prepared_input)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SMALLER_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const PART2_SMALLER_EXAMPLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    #[test]
    fn prepare_smaller_example() {
        let (grid, starting_position, movements) = prepare(SMALLER_EXAMPLE);
        assert_eq!(grid.dimensions, (8, 8).into());
        assert_eq!(starting_position, (2, 2).into());
        assert_eq!(movements.len(), 15);
    }
    #[test]
    fn part1_smaller_example() {
        assert_eq!(solve_part1(&prepare(SMALLER_EXAMPLE)), 2028);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE)), 10092);
    }
    #[test]
    fn part2_smaller_example() {
        assert_eq!(
            solve_part2(&prepare(PART2_SMALLER_EXAMPLE)),
            105 + 207 + 306
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&prepare(EXAMPLE)), 9021);
    }
}
