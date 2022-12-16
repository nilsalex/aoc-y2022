#![feature(byte_slice_trim_ascii)]

use std::collections::{HashMap, HashSet};

const INPUT: &[u8] = include_bytes!("input.txt");
// const INPUT: &[u8] = include_bytes!("input_test.txt");

type Vertex = (u8, u8);

#[derive(Debug)]
struct State {
    position: Vertex,
    opened: HashSet<Vertex>,
    cumulative_flow: usize,
    time_remaining: usize,
}

fn part1(input: &[u8]) -> usize {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut edges: Vec<(Vertex, Vertex)> = Vec::new();
    let mut rates: HashMap<Vertex, usize> = HashMap::new();

    for line in input.trim_ascii_end().split(|byte| *byte == b'\n') {
        let mut split = line.split(|byte| *byte == b';');
        let first_half = split.next().unwrap();
        let vertex = (first_half[6], first_half[7]);
        let rate = std::str::from_utf8(&first_half[23..]).unwrap().parse().unwrap();

        vertices.push(vertex);
        rates.insert(vertex, rate);

        let second_half = &split.next().unwrap()[23..];
        for next_vertex in second_half.split(|byte| *byte == b',') {
            let next_vertex = next_vertex.trim_ascii_start();
            edges.push((vertex, (next_vertex[0], next_vertex[1])));
        }
    }

    // Floyd-Warshall

    let mut distances: HashMap<(Vertex, Vertex), usize> = HashMap::new();

    for (start, end) in edges {
        distances.insert((start, end), 1);
    }

    for vertex_k in vertices.iter() {
        for vertex_i in vertices.iter() {
            for vertex_j in vertices.iter() {
                let d_ij = distances.get(&(*vertex_i, *vertex_j)).unwrap_or(&10000);
                let d_ik = distances.get(&(*vertex_i, *vertex_k)).unwrap_or(&10000);
                let d_kj = distances.get(&(*vertex_k, *vertex_j)).unwrap_or(&10000);
                if *d_ij > *d_ik + *d_kj {
                    distances.insert((*vertex_i, *vertex_j), d_ik + d_kj);
                }
            }
        }
    }

    let nonzero_vertices = vertices
        .iter()
        .cloned()
        .filter(|v| rates.get(v).unwrap() != &0)
        .collect::<Vec<Vertex>>();

    let initial_state = State {
        position: (b'A', b'A'),
        opened: HashSet::new(),
        time_remaining: 30,
        cumulative_flow: 0,
    };

    *dfs(&initial_state, &nonzero_vertices, &distances, &rates).iter().max().unwrap()
}

fn dfs(state: &State, vertices: &[Vertex], distances: &HashMap<(Vertex, Vertex), usize>, rates: &HashMap<Vertex, usize>) -> Vec<usize> {
    let next_states = next_states(state, vertices, distances, rates);

    if next_states.is_empty() {
        vec![state.cumulative_flow]
    } else {
        next_states
            .iter()
            .flat_map(|s| dfs(s, vertices, distances, rates))
            .collect()
    }
}

fn next_states(state: &State, vertices: &[Vertex], distances: &HashMap<(Vertex, Vertex), usize>, rates: &HashMap<Vertex, usize>) -> Vec<State> {
    vertices
        .iter()
        .filter(|v| !state.opened.contains(v))
        .filter_map(|v| {
            let v_distance = distances.get(&(state.position, *v)).unwrap();
            let v_rate = rates.get(v).unwrap();
            if state.time_remaining > v_distance + 1 {
                let mut new_opened = state.opened.clone();
                new_opened.insert(*v);
                let new_time_remaining = state.time_remaining - v_distance - 1;
                let new_cumulative_flow = state.cumulative_flow + new_time_remaining * v_rate;

                Some(State {
                    position: *v,
                    opened: new_opened,
                    cumulative_flow: new_cumulative_flow,
                    time_remaining: new_time_remaining,
                })
            } else {
                None
            }
        })
        .collect()
}

fn part2(_input: &[u8]) -> usize {
    0
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
