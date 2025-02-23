use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::array::TryFromSliceError;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use winnow::ascii::{alphanumeric1, dec_uint};
use winnow::combinator::{alt, opt, preceded, separated_pair};
use winnow::error::ContextError;
use winnow::token::take_while;
use winnow::{ModalResult, Parser};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct Wire {
    name: [u8; 3],
}
impl Display for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.name).unwrap())
    }
}
impl Debug for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.name).unwrap())
    }
}
impl FromStr for Wire {
    type Err = TryFromSliceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes().try_into().map(|name| Wire { name })
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Xor,
}

type PreparedInput = (FxHashMap<Wire, bool>, Vec<(Wire, Wire, Operator, Wire)>);

fn wire(input: &mut &str) -> ModalResult<Wire> {
    alphanumeric1
        .try_map(|s: &str| s.as_bytes().try_into().map(|name| Wire { name }))
        .parse_next(input)
}

fn starting_value(input: &mut &str) -> ModalResult<(Wire, bool)> {
    separated_pair(wire, ": ", alt(('0'.value(false), '1'.value(true)))).parse_next(input)
}
fn operation(input: &mut &str) -> ModalResult<(Wire, Wire, Operator, Wire)> {
    (
        wire,
        ' ',
        alt((
            "AND".value(Operator::And),
            "OR".value(Operator::Or),
            "XOR".value(Operator::Xor),
        )),
        ' ',
        wire,
        " -> ",
        wire,
    )
        .map(|(a, _, operator, _, b, _, target)| (a, b, operator, target))
        .parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    let mut sections = input.split("\n\n");
    let starting_values_section = sections.next().unwrap();
    let operations_section = sections.next().unwrap();
    assert!(sections.next().is_none());

    let starting_values = starting_values_section
        .lines()
        .map(|l| starting_value.parse(l).unwrap_or_else(|e| panic!("{}", e)))
        .collect();
    let operations = operations_section
        .lines()
        .map(|l| operation.parse(l).unwrap_or_else(|e| panic!("{}", e)))
        .collect();

    (starting_values, operations)
}

fn wire_offset(wire: &Wire) -> u8 {
    preceded(
        take_while(0.., '0'),
        opt(dec_uint::<_, u8, ContextError>).map(|num| num.unwrap_or(0)),
    )
    .parse(&wire.name[1..3])
    .unwrap_or_else(|e| panic!("{}", e))
}

fn calc_number(letter: u8, values: &FxHashMap<Wire, bool>) -> u64 {
    let mut number = 0u64;
    values.iter().for_each(|(wire, value)| {
        if wire.name[0] == letter && *value {
            number |= 1 << wire_offset(wire);
        }
    });
    number
}

fn solve_part1(input: &PreparedInput) -> u64 {
    let (starting_values, operations) = input;
    let mut mapping = FxHashMap::default();
    operations
        .iter()
        .enumerate()
        .for_each(|(i, (a, b, _, target))| {
            mapping.entry(*a).or_insert_with(Vec::new).push(i);
            mapping.entry(*b).or_insert_with(Vec::new).push(i);
            mapping.entry(*target).or_insert_with(Vec::new);
        });

    let mut values = starting_values.clone();
    let mut previous_iteration = starting_values.keys().cloned().collect::<FxHashSet<Wire>>();

    loop {
        let consider_operations = previous_iteration
            .into_iter()
            .flat_map(|wire| mapping.get(&wire).unwrap())
            .cloned()
            .filter(|i| {
                let op = &operations[*i];
                values.contains_key(&op.0) && values.contains_key(&op.1)
            })
            .collect::<FxHashSet<usize>>();

        if consider_operations.is_empty() {
            break;
        }

        assert!(!consider_operations.is_empty());

        let mut next_pending = FxHashSet::default();
        consider_operations.into_iter().for_each(|i| {
            let op = &operations[i];
            let value = match op.2 {
                Operator::And => values[&op.0] && values[&op.1],
                Operator::Or => values[&op.0] || values[&op.1],
                Operator::Xor => values[&op.0] ^ values[&op.1],
            };
            values.insert(op.3, value);
            next_pending.insert(op.3);
        });

        previous_iteration = next_pending;
    }

    assert_eq!(values.len(), mapping.len());

    calc_number(b'z', &values)
}

fn solve_part2<const EXAMPLE: bool>(input: &PreparedInput) -> String {
    let (starting_values, operations) = input;

    let mut mapping = FxHashMap::default();
    operations
        .iter()
        .enumerate()
        .for_each(|(i, (a, b, _, target))| {
            mapping.entry(*a).or_insert_with(Vec::new).push(i);
            mapping.entry(*b).or_insert_with(Vec::new).push(i);
            mapping.entry(*target).or_insert_with(Vec::new);
        });

    let swaps: Vec<Wire> = if EXAMPLE {
        // bitwise AND
        let mut swaps: FxHashSet<Wire> = FxHashSet::default();
        operations.iter().for_each(|op| {
            assert_eq!(wire_offset(&op.0), wire_offset(&op.1));
            assert_eq!(op.2, Operator::And);
            if wire_offset(&op.0) != wire_offset(&op.3) {
                swaps.insert(op.3);
            }
        });
        swaps.into_iter().collect()
    } else {
        // +

        let mut operations = operations.clone();
        let swaps = [
            ("z32".parse().unwrap(), "tbt".parse().unwrap()),
            ("z12".parse().unwrap(), "kth".parse().unwrap()),
            ("z26".parse().unwrap(), "gsd".parse().unwrap()),
            ("vpm".parse().unwrap(), "qnf".parse().unwrap()),
        ];
        for (a, b) in swaps {
            operations.iter_mut().for_each(|op| {
                if op.3 == a {
                    op.3 = b;
                } else if op.3 == b {
                    op.3 = a;
                }
            });
        }

        let mut previous_iteration = starting_values.keys().cloned().collect::<FxHashSet<Wire>>();
        let mut values = starting_values.clone();

        let expected_sum = calc_number(b'x', &values) + calc_number(b'y', &values);

        loop {
            // println!("Previous iteration: {:?}", previous_iteration);
            let consider_operations = previous_iteration
                .into_iter()
                .flat_map(|wire| mapping.get(&wire).unwrap())
                .cloned()
                .filter(|i| {
                    let op = &operations[*i];
                    values.contains_key(&op.0) && values.contains_key(&op.1)
                })
                .collect::<FxHashSet<usize>>();

            if consider_operations.is_empty() {
                break;
            }

            let mut next_pending = FxHashSet::default();
            consider_operations.into_iter().for_each(|i| {
                let op = &operations[i];
                let value = match op.2 {
                    Operator::And => values[&op.0] && values[&op.1],
                    Operator::Or => values[&op.0] || values[&op.1],
                    Operator::Xor => values[&op.0] ^ values[&op.1],
                };

                if op.3.name[0] == b'z' {
                    let offset = wire_offset(&op.3);
                    let expected_value = (expected_sum >> offset) & 1 == 1;
                    if value != expected_value {
                        println!("Mismatch: {}", op.3);
                    }
                }

                values.insert(op.3, value);
                next_pending.insert(op.3);
            });

            previous_iteration = next_pending;
        }

        assert_eq!(expected_sum, calc_number(b'z', &values));
        swaps.into_iter().flat_map(|(a, b)| [a, b]).collect()
    };

    let mut swaps = swaps.into_iter().collect::<Vec<_>>();
    swaps.sort_unstable();
    swaps.iter().join(",")
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input)),
        ctx.measure("part2", || solve_part2::<false>(&input)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(EXAMPLE_INPUT).0.len(), 10);
        assert_eq!(prepare(EXAMPLE_INPUT).1.len(), 36);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 2024);
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2::<true>(&prepare(
                "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"
            )),
            "z00,z01,z02,z05"
        );
    }
}
