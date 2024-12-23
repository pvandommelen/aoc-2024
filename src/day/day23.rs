use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use crate::util::solver::solve_priority;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::ControlFlow;
use winnow::combinator::separated_pair;
use winnow::{PResult, Parser};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Computer {
    name: [u8; 2],
}
impl Display for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.name).unwrap())
    }
}

fn computer(input: &mut &str) -> PResult<Computer> {
    winnow::ascii::alpha1
        .verify_map(|s: &str| s.as_bytes().try_into().ok())
        .map(|name| Computer { name })
        .parse_next(input)
}

fn line(input: &mut &str) -> PResult<(Computer, Computer)> {
    separated_pair(computer, '-', computer).parse_next(input)
}

fn parse(input: &str) -> Vec<(Computer, Computer)> {
    input
        .lines()
        .map(|l| line.parse(l).unwrap_or_else(|e| panic!("{}", e)))
        .collect()
}

struct Graph {
    nodes: FxHashMap<Computer, FxHashSet<Computer>>,
}

type PreparedInput = Graph;
fn prepare(parsed: Vec<(Computer, Computer)>) -> Graph {
    let mut graph = Graph {
        nodes: FxHashMap::default(),
    };
    parsed.into_iter().for_each(|(a, b)| {
        graph.nodes.entry(a).or_default().insert(b);
        graph.nodes.entry(b).or_default().insert(a);
    });
    graph
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Set {
    set: Vec<Computer>,
}
impl FromIterator<Computer> for Set {
    fn from_iter<I: IntoIterator<Item = Computer>>(iter: I) -> Self {
        let mut set = iter.into_iter().collect::<Vec<_>>();
        set.sort_unstable();
        Set { set }
    }
}
impl Set {
    fn intersect(&self, set: &FxHashSet<Computer>) -> Self {
        Self::from_iter(self.set.iter().filter(|c| set.contains(c)).cloned())
    }
}

fn solve_part1(graph: &PreparedInput) -> usize {
    let sets = graph
        .nodes
        .iter()
        .filter(|(computer, _)| computer.name[0] == b't')
        .flat_map(|(root, connected)| {
            connected.iter().flat_map(|con| {
                graph.nodes[con]
                    .intersection(connected)
                    .map(|third| Set::from_iter([*root, *con, *third]))
            })
        })
        .collect::<FxHashSet<_>>();

    sets.len()
}

#[derive(Eq, PartialEq)]
struct State {
    set: Set,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.set.set.len().cmp(&other.set.set.len())
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part2(graph: &PreparedInput) -> String {
    let computer_sets = graph
        .nodes
        .iter()
        .map(|(computer, connected)| {
            let candidate = Set::from_iter(connected.iter().chain([computer]).cloned());
            (*computer, candidate)
        })
        .collect::<FxHashMap<_, _>>();
    let candidates = computer_sets.values().map(|set| State { set: set.clone() });

    let computer_sets = graph
        .nodes
        .iter()
        .map(|(computer, connected)| {
            let mut set = connected.clone();
            set.insert(*computer);
            (*computer, set)
        })
        .collect::<FxHashMap<_, _>>();

    let state = solve_priority(
        |stack, s| {
            if s.set.set.iter().all(|computer| {
                let intersection = s.set.intersect(&computer_sets[computer]);

                if intersection.set.len() == s.set.set.len() {
                    true
                } else {
                    stack.push(State { set: intersection });
                    false
                }
            }) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        },
        candidates.collect_vec(),
    );
    let set = state.unwrap().set;

    set.set.into_iter().join(",")
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("parse", || parse(input));
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

    const EXAMPLE_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    #[test]
    fn prepare_example() {
        assert_eq!(parse(EXAMPLE_INPUT).len(), 32);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(parse(EXAMPLE_INPUT))), 7);
    }
    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&prepare(parse(EXAMPLE_INPUT))), "co,de,ka,ta");
    }
}
