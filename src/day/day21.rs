use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use crate::util::position::{Direction, Position};
use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

type PreparedInput = Vec<(usize, Vec<u8>)>;

fn prepare(input: &str) -> PreparedInput {
    input
        .lines()
        .map(|l| (l[0..3].parse::<usize>().unwrap(), l.as_bytes().to_owned()))
        .collect()
}

fn char_to_position(c: &u8) -> Position {
    match c {
        b'1'..=b'9' => {
            let num = c - b'1';
            Position((2 - num / 3) as usize, (num % 3) as usize)
        }
        b'0' => Position(3, 1),
        b'A' => Position(3, 2),
        _ => panic!(),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum DirectionalKey {
    Direction(Direction),
    A,
}

fn directional_key_to_position(key: &DirectionalKey) -> Position {
    match key {
        DirectionalKey::Direction(direction) => match direction {
            Direction::Up => Position(0, 1),
            Direction::Right => Position(1, 2),
            Direction::Down => Position(1, 1),
            Direction::Left => Position(1, 0),
        },
        DirectionalKey::A => Position(0, 2),
    }
}

fn numpad_sequence(sequence: &[u8]) -> Vec<Vec<Direction>> {
    let mut position = char_to_position(&b'A');
    sequence
        .iter()
        .map(char_to_position)
        .map(|char_position| {
            let offset = char_position - position;

            // Priority: Left -> Down/Up -> Right
            let mut ops = vec![];
            if (position.0 < 3 || char_position.1 > 0) && offset.1 < 0 {
                ops.extend(
                    std::iter::repeat::<Direction>(Direction::Left).take(-offset.1 as usize),
                );
            }
            if (position.1 > 0 || char_position.0 < 3) && offset.0 > 0 {
                ops.extend(std::iter::repeat::<Direction>(Direction::Down).take(offset.0 as usize));
            }
            if offset.0 < 0 {
                ops.extend(std::iter::repeat::<Direction>(Direction::Up).take(-offset.0 as usize));
            }
            if offset.1 > 0 {
                ops.extend(
                    std::iter::repeat::<Direction>(Direction::Right).take(offset.1 as usize),
                );
            }
            if !(position.0 < 3 || char_position.1 > 0) && offset.1 < 0 {
                ops.extend(
                    std::iter::repeat::<Direction>(Direction::Left).take(-offset.1 as usize),
                );
            }
            if !(position.1 > 0 || char_position.0 < 3) && offset.0 > 0 {
                ops.extend(std::iter::repeat::<Direction>(Direction::Down).take(offset.0 as usize));
            }
            position = char_position;
            ops
        })
        .collect()
}

fn directional_key_to_sequence(
    key: &DirectionalKey,
    position: &mut Position,
) -> ArrayVec<Direction, 3> {
    let key_position = directional_key_to_position(key);

    let offset = key_position - *position;

    // Priority: Left -> Down/Up -> Right
    let mut ops = ArrayVec::new();
    if (position.0 > 0 || key_position.1 > 0) && offset.1 < 0 {
        ops.extend(std::iter::repeat::<Direction>(Direction::Left).take(-offset.1 as usize));
    }
    if offset.0 > 0 {
        ops.extend(std::iter::repeat::<Direction>(Direction::Down).take(offset.0 as usize));
    }
    if (position.1 > 0 || key_position.0 > 0) && offset.0 < 0 {
        ops.extend(std::iter::repeat::<Direction>(Direction::Up).take(-offset.0 as usize));
    }
    if offset.1 > 0 {
        ops.extend(std::iter::repeat::<Direction>(Direction::Right).take(offset.1 as usize));
    }
    if !(position.0 > 0 || key_position.1 > 0) && offset.1 < 0 {
        ops.extend(std::iter::repeat::<Direction>(Direction::Left).take(-offset.1 as usize));
    }
    if !(position.1 > 0 || key_position.0 > 0) && offset.0 < 0 {
        ops.extend(std::iter::repeat::<Direction>(Direction::Up).take(-offset.0 as usize));
    }
    *position = key_position;
    ops
}

fn directional_sequence(sequence: &[Direction]) -> Vec<ArrayVec<Direction, 3>> {
    let mut position = directional_key_to_position(&DirectionalKey::A);
    sequence
        .iter()
        .map(|direction| DirectionalKey::Direction(*direction))
        .chain(std::iter::once(DirectionalKey::A))
        .map(|key| directional_key_to_sequence(&key, &mut position))
        .collect()
}

fn solve_iterations_single(sequence: &[u8], iterations: usize) -> usize {
    let parts = numpad_sequence(sequence);
    if iterations == 0 {
        return parts.into_iter().map(|sequence| sequence.len() + 1).sum();
    }

    let mut sequence_parts = parts.into_iter().fold(FxHashMap::default(), |map, part| {
        let sequence = directional_sequence(&part);
        sequence.into_iter().fold(map, |mut map, part| {
            *map.entry(part).or_insert(0) += 1;
            map
        })
    });

    let mut cache = FxHashMap::default();

    for _ in 1..iterations {
        let mut new_sequence_counts =
            FxHashMap::with_capacity_and_hasher(sequence_parts.len(), Default::default());
        for (part, part_count) in sequence_parts {
            let sequence = cache
                .entry(part.clone())
                .or_insert_with(|| directional_sequence(&part));
            sequence.iter().for_each(|part| {
                *new_sequence_counts.entry(part.to_owned()).or_insert(0) += part_count;
            });
        }
        sequence_parts = new_sequence_counts;
    }
    sequence_parts
        .iter()
        .map(|(part, count)| (part.len() + 1) * *count)
        .sum::<usize>()
}

fn solve_iterations(input: &PreparedInput, iterations: usize) -> usize {
    input
        .iter()
        .map(|(num, sequence)| num * solve_iterations_single(sequence, iterations))
        .sum()
}

fn solve_part1(input: &PreparedInput) -> usize {
    solve_iterations(input, 2)
}

fn solve_part2(input: &PreparedInput) -> usize {
    solve_iterations(input, 25)
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
    use rstest::*;

    const EXAMPLE_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 5);
    }

    #[test]
    fn first_robot_sequence_example() {
        assert_eq!(
            solve_iterations_single("029A".as_bytes(), 0),
            "<A^A>^^AvvvA".len()
        );
    }

    #[test]
    fn second_robot_sequence_example() {
        assert_eq!(
            solve_iterations_single("029A".as_bytes(), 1),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len()
        );
    }

    #[test]
    fn third_robot_sequence_example() {
        assert_eq!(
            solve_iterations_single("029A".as_bytes(), 2),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[rstest]
    #[case(
        "029A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
    )]
    #[case("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A")]
    #[case(
        "179A",
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    )]
    #[case(
        "456A",
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"
    )]
    #[case(
        "379A",
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    )]
    #[test]
    fn part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(solve_iterations_single(input.as_bytes(), 2), expected.len());
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 126384);
    }
}
