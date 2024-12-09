use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;

type PreparedInput = Vec<u8>;

fn prepare(input: &str) -> Vec<u8> {
    input.as_bytes().iter().map(|&b| b - b'0').collect()
}

fn solve_part1(input: &PreparedInput) -> usize {
    let required: usize = input.iter().step_by(2).map(|num| *num as usize).sum();
    let end = input.iter().rev().step_by(2);
    let mut end_ids = (0..(input.len() + 1) / 2)
        .rev()
        .zip(end)
        .flat_map(|(id, len)| std::iter::repeat_n(id, *len as usize));

    input
        .iter()
        .enumerate()
        .scan(0usize, |pos, (i, num)| {
            if *pos >= required {
                return None;
            }
            let up_to = (*pos + (*num as usize)).min(required);

            let value: usize = if i % 2 == 0 {
                // Block
                let block_id = i / 2;
                (*pos..up_to).map(|pos| pos * block_id).sum()
            } else {
                (*pos..up_to)
                    .map(|pos| {
                        let end_id = end_ids.next().unwrap();
                        pos * end_id
                    })
                    .sum()
            };

            *pos = up_to;

            Some(value)
        })
        .sum()
}

fn solve_part2(input: &PreparedInput) -> usize {
    let (mut blocks, mut empty) = input
        .iter()
        .enumerate()
        .scan(0usize, |pos, (i, num)| {
            let up_to = *pos + (*num as usize);
            let value = (i, *pos..up_to);
            *pos = up_to;
            Some(value)
        })
        .partition::<Vec<_>, _>(|(i, _)| *i % 2 == 0);

    for block in blocks.iter_mut().rev() {
        for space in empty.iter_mut() {
            let block_len = block.1.len();
            if space.1.len() >= block_len && space.1.start < block.1.start {
                block.1.start = space.1.start;
                block.1.end = space.1.start + block_len;
                space.1.start += block_len;
                break;
            }
        }
    }

    blocks
        .into_iter()
        .map(|(i, range)| range.map(|pos| pos * i / 2).sum::<usize>())
        .sum()
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

    const EXAMPLE_INPUT: &str = "2333133121414131402";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 19);
    }
    #[test]
    fn example_part1_simpler() {
        assert_eq!(
            solve_part1(&prepare("12345")),
            2 + 4 + 3 + 4 + 5 + 12 + 14 + 16
        );
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 1928);
    }
    #[test]
    fn example_part2_simpler() {
        assert_eq!(
            solve_part2(&prepare("12345")),
            3 + 4 + 5 + 20 + 22 + 24 + 26 + 28
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 2858);
    }
}
