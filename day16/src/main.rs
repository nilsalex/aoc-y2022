#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

const INPUT: &[u8] = include_bytes!("input.txt");
// const INPUT: &[u8] = include_bytes!("input_test.txt");

type Vertex = u16;
type Edge = (Vertex, Vertex);
type Distances = HashMap<Edge, usize>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct PackedValves(u32);

impl PackedValves {
    fn new() -> Self {
        Self(0)
    }

    fn set(&self, valve: u8) -> Self {
        Self(self.0 | (1 << valve))
    }

    fn is_set(&self, valve: u8) -> bool {
        self.0 & (1 << valve) != 0
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct StateV2 {
    cumulative_flow: usize,
    opened: PackedValves,
    positions: [u8; 2],
    times: [u8; 2],
}

impl StateV2 {
    fn apply_heuristics(&self, heuristics: &[Vec<BestValvesHeuristics>]) -> usize {
        let mut times = self.times;
        let mut opened = self.opened;
        let mut upper_bound = self.cumulative_flow;

        'next_valve: loop {
            for heuristics in &heuristics[times[0] as usize] {
                if !opened.is_set(heuristics.valve) {
                    times[0] -= heuristics.dist;
                    upper_bound += heuristics.flow as usize * times[0] as usize;
                    if times[0] < times[1] {
                        times.swap(0, 1)
                    }
                    opened = opened.set(heuristics.valve);
                    continue 'next_valve;
                }
            }
            break;
        }

        upper_bound
    }
}

#[derive(Debug)]
struct FullyConnectedGraph {
    num_vertices: usize,
    weights: Vec<u8>,
    values: Vec<u8>,
}

impl FullyConnectedGraph {
    fn get_weight(&self, v1: usize, v2: usize) -> u8 {
        self.weights[v1 * self.num_vertices + v2]
    }

    fn from_initial_graph(initial_graph: &InitialGraph) -> Self {
        let mut sorted_vertices = initial_graph.vertices.clone();
        sorted_vertices.sort_unstable();

        let vertex_map: HashMap<u8, u16> = sorted_vertices
            .iter()
            .filter(|v| *v == &0 || initial_graph.values.get(v).unwrap() != &0)
            .enumerate()
            .map(|(ix, v)| (ix as u8, *v))
            .collect();

        let num_vertices = vertex_map.len();
        let distances = initial_graph.distances();
        let mut weights: Vec<u8> = Vec::with_capacity(num_vertices * num_vertices);

        for v1 in 0..num_vertices as u8 {
            for v2 in 0..num_vertices as u8 {
                if v1 == v2 {
                    weights.push(0);
                } else {
                    weights.push(
                        *distances
                            .get(&(*vertex_map.get(&v1).unwrap(), *vertex_map.get(&v2).unwrap()))
                            .unwrap() as u8,
                    );
                }
            }
        }

        let values: Vec<u8> = (0..num_vertices as u8)
            .map(|v| {
                *initial_graph
                    .values
                    .get(vertex_map.get(&v).unwrap())
                    .unwrap() as u8
            })
            .collect();

        FullyConnectedGraph {
            num_vertices,
            weights,
            values,
        }
    }

    fn best_valves_heuristics(&self, max_time: usize) -> Vec<Vec<BestValvesHeuristics>> {
        (0..=max_time)
            .map(|time| {
                let mut best_for_time: Vec<BestValvesHeuristics> = (0..self.num_vertices)
                    .flat_map(|valve| {
                        let dists = (1..self.num_vertices)
                            .filter(|next_valve| *next_valve != valve)
                            .map(|next_valve| {
                                self.get_weight(next_valve, valve) + 1
                            });
                        let min_dist = dists.min().unwrap();
                        (time as u8 > min_dist).then_some(BestValvesHeuristics {
                            valve: valve as u8,
                            dist: min_dist,
                            flow: self.values[valve],
                        })
                    })
                    .collect();
                best_for_time.sort_by_key(|h| {
                    let BestValvesHeuristics {
                        valve: _,
                        dist,
                        flow,
                    } = h;
                    Reverse(*flow as usize * (time - *dist as usize))
                });
                best_for_time
            })
            .collect()
    }
}

#[derive(Debug)]
struct BestValvesHeuristics {
    valve: u8,
    dist: u8,
    flow: u8,
}

struct InitialGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    values: HashMap<Vertex, usize>,
}

impl InitialGraph {
    fn parse(bytes: &[u8]) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut edges: Vec<(Vertex, Vertex)> = Vec::new();
        let mut values: HashMap<Vertex, usize> = HashMap::new();
        let mut nonzero_vertices: Vec<Vertex> = Vec::new();

        for line in bytes.trim_ascii_end().split(|byte| *byte == b'\n') {
            let mut split = line.split(|byte| *byte == b';');
            let first_half = split.next().unwrap();
            let (char1, char2) = ((first_half[6] - b'A') as u16, (first_half[7] - b'A') as u16);
            let vertex = 26 * char1 + char2;
            vertices.push(vertex);

            let rate = std::str::from_utf8(&first_half[23..])
                .unwrap()
                .parse()
                .unwrap();
            if rate != 0 {
                nonzero_vertices.push(vertex);
            }

            values.insert(vertex, rate);

            let second_half = &split.next().unwrap()[23..];
            for next_vertex in second_half.split(|byte| *byte == b',') {
                let next_vertex = next_vertex.trim_ascii_start();
                let (char1, char2) = (
                    (next_vertex[0] - b'A') as u16,
                    (next_vertex[1] - b'A') as u16,
                );
                let next_vertex = 26 * char1 + char2;
                edges.push((vertex, next_vertex));
            }
        }

        InitialGraph {
            vertices,
            edges,
            values,
        }
    }

    fn distances(&self) -> Distances {
        let mut distances: Distances = HashMap::new();

        for (start, end) in self.edges.iter() {
            distances.insert((*start, *end), 1);
        }

        for vertex_k in self.vertices.iter() {
            for vertex_i in self.vertices.iter() {
                for vertex_j in self.vertices.iter() {
                    let d_ij = distances.get(&(*vertex_i, *vertex_j)).unwrap_or(&10000);
                    let d_ik = distances.get(&(*vertex_i, *vertex_k)).unwrap_or(&10000);
                    let d_kj = distances.get(&(*vertex_k, *vertex_j)).unwrap_or(&10000);
                    if *d_ij > *d_ik + *d_kj {
                        distances.insert((*vertex_i, *vertex_j), d_ik + d_kj);
                    }
                }
            }
        }

        distances
    }
}

fn max_cumulative_flow(graph: &FullyConnectedGraph, initial_state: &StateV2) -> usize {
    let heuristics = graph.best_valves_heuristics(30);

    let mut visited: HashSet<StateV2> = HashSet::new();
    let mut queue: BinaryHeap<(usize, StateV2)> = BinaryHeap::new();
    let mut best = 0;

    queue.push((usize::MAX, initial_state.clone()));

    while let Some((upper, state)) = queue.pop() {
        if upper <= best {
            return best;
        }

        if !visited.insert(StateV2 {
            cumulative_flow: 0,
            ..state
        }) {
            continue;
        }

        for (next, flow) in graph.values.iter().enumerate().skip(1) {
            let dist = graph.get_weight(state.positions[0] as usize, next);
            let next = next as u8;

            if state.times[0] > dist + 1 && !state.opened.is_set(next) {
                let next_time = state.times[0] - dist - 1;
                let mut next_state = StateV2 {
                    cumulative_flow: state.cumulative_flow + *flow as usize * next_time as usize,
                    opened: state.opened.set(next),
                    positions: [next, state.positions[1]],
                    times: [next_time, state.times[1]],
                };
                if next_state.times[0] < next_state.times[1] {
                    next_state.positions.swap(0, 1);
                    next_state.times.swap(0, 1)
                }
                best = best.max(next_state.cumulative_flow);
                let upper = next_state.apply_heuristics(&heuristics);
                if upper > best {
                    queue.push((upper, next_state))
                }
            }
        }
    }

    best
}

fn part1(input: &[u8]) -> usize {
    let initial_graph = InitialGraph::parse(input);
    let graph = FullyConnectedGraph::from_initial_graph(&initial_graph);

    let initial_state = StateV2 {
        cumulative_flow: 0,
        opened: PackedValves::new().set(0),
        positions: [0, 0],
        times: [30, 0],
    };

    max_cumulative_flow(&graph, &initial_state)
}

fn part2(input: &[u8]) -> usize {
    let initial_graph = InitialGraph::parse(input);
    let graph = FullyConnectedGraph::from_initial_graph(&initial_graph);

    let initial_state = StateV2 {
        cumulative_flow: 0,
        opened: PackedValves::new().set(0),
        positions: [0, 0],
        times: [26, 26],
    };

    max_cumulative_flow(&graph, &initial_state)
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2183)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2911)
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| InitialGraph::parse(INPUT))
    }

    #[bench]
    fn bench_distances(b: &mut Bencher) {
        let graph = InitialGraph::parse(INPUT);
        b.iter(|| graph.distances())
    }

    #[bench]
    fn bench_construct_full_graph(b: &mut Bencher) {
        let initial_graph = InitialGraph::parse(INPUT);
        b.iter(|| FullyConnectedGraph::from_initial_graph(&initial_graph))
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT))
    }
}
