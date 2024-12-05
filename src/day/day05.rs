use crate::solution::SolutionTuple;
use crate::util::intset::ArraySet;
use crate::util::measure::MeasureContext;

struct PreparedInput {
    /// Indexed by second number to first number.
    page_ordering_rules: Vec<ArraySet<100>>,
    updates: Vec<Vec<u8>>,
}

fn parse_two_char_num(b: &[u8]) -> u8 {
    (b[0] - b'0') * 10 + (b[1] - b'0')
}

fn prepare(input: &str) -> PreparedInput {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    assert_eq!(sections.len(), 2);

    PreparedInput {
        page_ordering_rules: sections[0].lines().fold(
            vec![ArraySet::<100>::new(); 100],
            |mut map, l| {
                let bytes = l.as_bytes();
                assert_eq!(bytes.len(), 5);
                let first = parse_two_char_num(&bytes[0..2]);
                let second = parse_two_char_num(&bytes[3..5]);
                map[second as usize].insert(first);
                map
            },
        ),
        updates: sections[1]
            .lines()
            .map(|l| {
                let chunks = l.as_bytes().chunks_exact(3);

                let mut v = Vec::with_capacity(chunks.len() + 1);
                let remainder = chunks.remainder();
                v.extend(chunks.map(parse_two_char_num));
                v.push(parse_two_char_num(remainder));
                v
            })
            .collect(),
    }
}

fn find_last_mismatch(input: &PreparedInput, u: &[u8], i: usize) -> Option<usize> {
    let num = u[i];
    let firsts = &input.page_ordering_rules[num as usize];
    u[i..]
        .iter()
        .rposition(|num| firsts.contains(num))
        .map(|idx| idx + i)
}

fn fix_sort_halfway(input: &PreparedInput, u: &[u8], modified: &mut bool) -> u8 {
    let mut u = u.to_vec();

    let mut i = 0;
    loop {
        let idx_furthest = find_last_mismatch(input, &u, i);

        if let Some(idx_furthest) = idx_furthest {
            *modified = true;
            u[i..idx_furthest + 1].rotate_left(1);
        } else {
            if i == (u.len() - 1) / 2 {
                return u[i];
            }
            i += 1;
        }
    }
}

fn solve_both(input: &PreparedInput) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    input.updates.iter().for_each(|u| {
        assert_eq!(u.len() % 2, 1);

        let mut modified = false;
        let halfway = fix_sort_halfway(input, u, &mut modified);

        if !modified {
            p1 += halfway as u32;
        } else {
            p2 += halfway as u32;
        }
    });
    (p1, p2)
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn example_prepare() {
        let input = prepare(EXAMPLE_INPUT);
        assert_eq!(
            input
                .page_ordering_rules
                .iter()
                .map(|rule| rule.len())
                .sum::<usize>(),
            21
        );
        assert_eq!(input.updates.len(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 143);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 123);
    }
}
