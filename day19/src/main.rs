use std::thread;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("input_test.txt");

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    ore_robot_costs_ore: u32,
    clay_robot_costs_ore: u32,
    obsidian_robot_costs_ore: u32,
    obsidian_robot_costs_clay: u32,
    geode_robot_costs_ore: u32,
    geode_robot_costs_obsidian: u32,
}

#[derive(Debug, Clone)]
struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

struct State {
    inventory: Inventory,
    depth: usize,
}

fn next_states(state: &State, blueprint: &Blueprint) -> Vec<State> {
//    if state.depth < 8 {
//        for _ in 0..state.depth {
//            print!("  ");
//        }
//        println!("{}", state.depth);
//    }
    if state.depth == 24 {
        return Vec::new();
    }

    let mut next_states = Vec::new();

    if state.inventory.ore >= blueprint.geode_robot_costs_ore && state.inventory.obsidian >= blueprint.geode_robot_costs_obsidian {
        next_states.push(State {
            depth: state.depth + 1,
            inventory: Inventory {
                ore: state.inventory.ore + state.inventory.ore_robots - blueprint.geode_robot_costs_ore,
                clay: state.inventory.clay + state.inventory.clay_robots,
                obsidian: state.inventory.obsidian + state.inventory.obsidian_robots - blueprint.geode_robot_costs_obsidian,
                geodes: state.inventory.geodes + state.inventory.geode_robots,
                ore_robots: state.inventory.ore_robots,
                clay_robots: state.inventory.clay_robots,
                obsidian_robots: state.inventory.obsidian_robots,
                geode_robots: state.inventory.geode_robots + 1,
            },
        });
    } else {
        next_states.push(State {
            depth: state.depth + 1,
            inventory: Inventory {
                ore: state.inventory.ore + state.inventory.ore_robots,
                clay: state.inventory.clay + state.inventory.clay_robots,
                obsidian: state.inventory.obsidian + state.inventory.obsidian_robots,
                geodes: state.inventory.geodes + state.inventory.geode_robots,
                ore_robots: state.inventory.ore_robots,
                clay_robots: state.inventory.clay_robots,
                obsidian_robots: state.inventory.obsidian_robots,
                geode_robots: state.inventory.geode_robots,
            },
        });
        if state.inventory.ore >= blueprint.obsidian_robot_costs_ore && state.inventory.clay >= blueprint.obsidian_robot_costs_clay {
            next_states.push(State {
                depth: state.depth + 1,
                inventory: Inventory {
                    ore: state.inventory.ore + state.inventory.ore_robots - blueprint.obsidian_robot_costs_ore,
                    clay: state.inventory.clay + state.inventory.clay_robots - blueprint.obsidian_robot_costs_clay,
                    obsidian: state.inventory.obsidian + state.inventory.obsidian_robots,
                    geodes: state.inventory.geodes + state.inventory.geode_robots,
                    ore_robots: state.inventory.ore_robots,
                    clay_robots: state.inventory.clay_robots,
                    obsidian_robots: state.inventory.obsidian_robots + 1,
                    geode_robots: state.inventory.geode_robots,
                },
            });
        }
        if state.inventory.ore >= blueprint.clay_robot_costs_ore {
            next_states.push(State {
                depth: state.depth + 1,
                inventory: Inventory {
                    ore: state.inventory.ore + state.inventory.ore_robots - blueprint.clay_robot_costs_ore,
                    clay: state.inventory.clay + state.inventory.clay_robots,
                    obsidian: state.inventory.obsidian + state.inventory.obsidian_robots,
                    geodes: state.inventory.geodes + state.inventory.geode_robots,
                    ore_robots: state.inventory.ore_robots,
                    clay_robots: state.inventory.clay_robots + 1,
                    obsidian_robots: state.inventory.obsidian_robots,
                    geode_robots: state.inventory.geode_robots,
                },
            });
        }
        if state.inventory.ore >= blueprint.ore_robot_costs_ore {
            next_states.push(State {
                depth: state.depth + 1,
                inventory: Inventory {
                    ore: state.inventory.ore + state.inventory.ore_robots - blueprint.ore_robot_costs_ore,
                    clay: state.inventory.clay + state.inventory.clay_robots,
                    obsidian: state.inventory.obsidian + state.inventory.obsidian_robots,
                    geodes: state.inventory.geodes + state.inventory.geode_robots,
                    ore_robots: state.inventory.ore_robots + 1,
                    clay_robots: state.inventory.clay_robots,
                    obsidian_robots: state.inventory.obsidian_robots,
                    geode_robots: state.inventory.geode_robots,
                },
            });
        }
    }

    next_states
}

fn dfs(state: &State, blueprint: &Blueprint) -> u32 {
    let next_states = next_states(state, blueprint);

    if next_states.is_empty() {
        state.inventory.geodes
    } else {
        next_states
            .iter()
            .map(|state| dfs(state, blueprint))
            .max()
            .unwrap()
    }
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();

    let blueprints = input.trim_end().lines().map(|line| {
        let cap = re.captures(line).unwrap();
        Blueprint {
            id: cap[1].parse().unwrap(),
            ore_robot_costs_ore: cap[2].parse().unwrap(),
            clay_robot_costs_ore: cap[3].parse().unwrap(),
            obsidian_robot_costs_ore: cap[4].parse().unwrap(),
            obsidian_robot_costs_clay: cap[5].parse().unwrap(),
            geode_robot_costs_ore: cap[6].parse().unwrap(),
            geode_robot_costs_obsidian: cap[7].parse().unwrap(),
        }
    }).collect::<Vec<Blueprint>>();

    let mut handles = Vec::new();

    for chunk in blueprints.chunks((blueprints.len() / 4).max(1)) {
        let cloned_chunk = Vec::from(chunk);
        let handle = thread::spawn(move || {
            let mut chunk_result: usize = 0;
            for blueprint in cloned_chunk {
                let initial_state = State {
                    depth: 0,
                    inventory: Inventory
                    {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geodes: 0,
                        ore_robots: 1,
                        clay_robots: 0,
                        obsidian_robots: 0,
                        geode_robots: 0,
                    },
                };
                let max_geodes = dfs(&initial_state, &blueprint);
                println!("blueprint {}: {} geodes", blueprint.id, max_geodes);
                chunk_result += blueprint.id as usize * max_geodes as usize;
            }
            chunk_result
        });
        handles.push(handle);
    }

    let mut result: usize = 0;
    for handle in handles {
        result += handle.join().unwrap()
    }

    result
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
