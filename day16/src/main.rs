#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("input.txt");
// const INPUT: &[u8] = include_bytes!("input_test.txt");

type Vertex = u16;
type Edge = (Vertex, Vertex);
type Distances = HashMap<Edge, usize>;

trait IsState {
    fn get_cumulative_flow(&self) -> usize;
}

#[derive(Debug)]
struct State {
    position: usize,
    opened: Vec<bool>,
    cumulative_flow: usize,
    time_remaining: u8,
}

impl IsState for State {
    fn get_cumulative_flow(&self) -> usize {
        self.cumulative_flow
    }
}

#[derive(Debug)]
struct ExtendedState {
    position_1: usize,
    position_2: usize,
    opened: Vec<bool>,
    cumulative_flow: usize,
    time_remaining_1: u8,
    time_remaining_2: u8,
    depth: u8,
}

impl IsState for ExtendedState {
    fn get_cumulative_flow(&self) -> usize {
        self.cumulative_flow
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

    fn get_value(&self, v: usize) -> u8 {
        self.values[v]
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
                    weights.push(*distances.get(&(*vertex_map.get(&v1).unwrap(), *vertex_map.get(&v2).unwrap())).unwrap() as u8);
                }
            }
        }

        let values: Vec<u8> = (0..num_vertices as u8)
            .map(|v| *initial_graph.values.get(vertex_map.get(&v).unwrap()).unwrap() as u8)
            .collect();

        FullyConnectedGraph { num_vertices, weights, values }
    }
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

            let rate = std::str::from_utf8(&first_half[23..]).unwrap().parse().unwrap();
            if rate != 0 {
                nonzero_vertices.push(vertex);
            }

            values.insert(vertex, rate);

            let second_half = &split.next().unwrap()[23..];
            for next_vertex in second_half.split(|byte| *byte == b',') {
                let next_vertex = next_vertex.trim_ascii_start();
                let (char1, char2) = ((next_vertex[0] - b'A') as u16, (next_vertex[1] - b'A') as u16);
                let next_vertex = 26 * char1 + char2;
                edges.push((vertex, next_vertex));
            }
        }

        InitialGraph { vertices, edges, values }
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

fn dfs<S>(state: &S, next_fn: &impl Fn(&S) -> Vec<S>) -> Vec<usize>
    where S: IsState {
    let next_states = next_fn(state);

    if next_states.is_empty() {
        vec![state.get_cumulative_flow()]
    } else {
        next_states
            .iter()
            .flat_map(|s| dfs(s, next_fn))
            .collect()
    }
}

fn next_states(state: &State, graph: &FullyConnectedGraph) -> Vec<State> {
    (0..graph.num_vertices)
        .filter(|v| !state.opened[*v])
        .filter_map(|position| {
            let (weight, rate) = (graph.get_weight(state.position, position), graph.get_value(position));

            if state.time_remaining > weight + 1 {
                let mut opened = state.opened.clone();
                opened[position] = true;

                let time_remaining = state.time_remaining - weight - 1;
                let cumulative_flow = state.cumulative_flow + time_remaining as usize * rate as usize;

                Some(State { position, opened, cumulative_flow, time_remaining })
            } else {
                None
            }
        })
        .collect()
}

fn next_extended_states(state: &ExtendedState, graph: &FullyConnectedGraph) -> Vec<ExtendedState> {
    let mut next_states = Vec::new();

    if state.time_remaining_1 >= state.time_remaining_2 {
        next_states.extend((0..graph.num_vertices)
            .filter(|v| !state.opened[*v])
            .filter_map(|v| {
                let (weight, rate) = (graph.get_weight(state.position_1, v), graph.get_value(v));

                if state.time_remaining_1 > weight + 1 {
                    let mut opened = state.opened.clone();
                    opened[v] = true;

                    let time_remaining = state.time_remaining_1 - weight - 1;
                    let cumulative_flow = state.cumulative_flow + time_remaining as usize * rate as usize;

                    Some(ExtendedState {
                        position_1: v,
                        position_2: state.position_2,
                        opened,
                        cumulative_flow,
                        time_remaining_1: time_remaining,
                        time_remaining_2: state.time_remaining_2,
                        depth: state.depth + 1,
                    })
                } else {
                    None
                }
            }));
    }

    if next_states.is_empty() {
        next_states.extend((0..graph.num_vertices)
            .filter(|v| !state.opened[*v])
            .filter_map(|v| {
                let (weight, rate) = (graph.get_weight(state.position_2, v), graph.get_value(v));

                if state.time_remaining_2 > weight + 1 {
                    let mut opened = state.opened.clone();
                    opened[v] = true;

                    let time_remaining = state.time_remaining_2 - weight - 1;
                    let cumulative_flow = state.cumulative_flow + time_remaining as usize * rate as usize;

                    Some(ExtendedState {
                        position_1: state.position_1,
                        position_2: v,
                        opened,
                        cumulative_flow,
                        time_remaining_1: state.time_remaining_1,
                        time_remaining_2: time_remaining,
                        depth: state.depth + 1,
                    })
                } else {
                    None
                }
            }));
    }

    if state.depth < 3 {
        for _ in 0..state.depth {
            print!(" ")
        }
        println!("depth {}: {} branches", state.depth, next_states.len())
    }

    next_states
}

fn part1(input: &[u8]) -> usize {
    let initial_graph = InitialGraph::parse(input);
    let graph = FullyConnectedGraph::from_initial_graph(&initial_graph);

    let initial_state = State {
        position: 0,
        opened: vec![false; graph.num_vertices],
        time_remaining: 30,
        cumulative_flow: 0,
    };

    *dfs(&initial_state, &|state| next_states(state, &graph)).iter().max().unwrap()
}

fn part2(input: &[u8]) -> usize {
    let initial_graph = InitialGraph::parse(input);
    let graph = FullyConnectedGraph::from_initial_graph(&initial_graph);

    let initial_state = ExtendedState {
        position_1: 0,
        position_2: 0,
        opened: vec![false; graph.num_vertices],
        time_remaining_1: 26,
        time_remaining_2: 26,
        cumulative_flow: 0,
        depth: 0,
    };

    *dfs(&initial_state, &|state| next_extended_states(state, &graph)).iter().max().unwrap()
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
