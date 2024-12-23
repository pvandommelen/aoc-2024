use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use crate::util::solver::{Stack, solve_priority_dedup};
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
    nodes: Vec<Computer>,
    edge_map: Vec<FxHashSet<usize>>,
}

type PreparedInput = Graph;
fn prepare(parsed: Vec<(Computer, Computer)>) -> Graph {
    let mut nodes = vec![];
    let mut nodes_map = FxHashMap::default();
    let mut edge_map = vec![];

    parsed.into_iter().for_each(|(a, b)| {
        let idx_a = *nodes_map.entry(a).or_insert_with(|| {
            nodes.push(a);
            edge_map.push(FxHashSet::default());
            nodes.len() - 1
        });
        let idx_b = *nodes_map.entry(b).or_insert_with(|| {
            nodes.push(b);
            edge_map.push(FxHashSet::default());
            nodes.len() - 1
        });
        edge_map[idx_a].insert(idx_b);
        edge_map[idx_b].insert(idx_a);
    });
    Graph { nodes, edge_map }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct OrderedSet {
    set: Vec<usize>,
}
impl FromIterator<usize> for OrderedSet {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let mut set = iter.into_iter().collect::<Vec<_>>();
        set.sort_unstable();
        OrderedSet { set }
    }
}
impl OrderedSet {
    fn intersect(&self, set: &FxHashSet<usize>) -> Self {
        let mut v = self.set.clone();
        v.retain(|c| set.contains(c));
        Self { set: v }
    }
}

fn solve_part1(graph: &PreparedInput) -> usize {
    let sets = graph
        .nodes
        .iter()
        .positions(|computer| computer.name[0] == b't')
        .flat_map(|i| {
            graph.edge_map[i].iter().flat_map(move |con| {
                graph.edge_map[*con]
                    .intersection(&graph.edge_map[i])
                    .map(move |third| OrderedSet::from_iter([i, *con, *third]))
            })
        })
        .collect::<FxHashSet<_>>();

    sets.len()
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    set: OrderedSet,
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
        .edge_map
        .iter()
        .enumerate()
        .map(|(computer, connected)| {
            let candidate = OrderedSet::from_iter(connected.iter().cloned().chain([computer]));
            (computer, candidate)
        })
        .collect::<FxHashMap<_, _>>();
    let candidates = computer_sets.values().map(|set| State { set: set.clone() });

    let computer_sets = graph
        .edge_map
        .iter()
        .enumerate()
        .map(|(computer, connected)| {
            let mut set = connected.clone();
            set.insert(computer);
            (computer, set)
        })
        .collect::<FxHashMap<_, _>>();

    let state = solve_priority_dedup(
        |stack, s| {
            let mut all_match = true;
            s.set.set.iter().for_each(|computer| {
                let intersection = s.set.intersect(&computer_sets[computer]);

                if intersection.set.len() != s.set.set.len() {
                    stack.push(State { set: intersection });
                    all_match = false;
                }
            });
            if all_match {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        },
        candidates.collect_vec(),
    );
    let set = state.unwrap().set;

    let mut set_names = set.set.into_iter().map(|i| graph.nodes[i]).collect_vec();
    set_names.sort_unstable();
    set_names.into_iter().join(",")
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
