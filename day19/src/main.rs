#![feature(test)]
extern crate test;

use regex::Regex;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    ore_robot_costs_ore: u32,
    clay_robot_costs_ore: u32,
    obsidian_robot_costs_ore: u32,
    obsidian_robot_costs_clay: u32,
    geode_robot_costs_ore: u32,
    geode_robot_costs_obsidian: u32,
    max_costs_ore: u32,
}

impl Blueprint {
    fn required_ore(&self, robot: &Robot) -> u32 {
        match robot {
            Robot::Ore => self.ore_robot_costs_ore,
            Robot::Clay => self.clay_robot_costs_ore,
            Robot::Obsidian => self.obsidian_robot_costs_ore,
            Robot::Geode => self.geode_robot_costs_ore,
        }
    }

    fn required_clay(&self, robot: &Robot) -> u32 {
        match robot {
            Robot::Ore => 0,
            Robot::Clay => 0,
            Robot::Obsidian => self.obsidian_robot_costs_clay,
            Robot::Geode => 0,
        }
    }

    fn required_obsidian(&self, robot: &Robot) -> u32 {
        match robot {
            Robot::Ore => 0,
            Robot::Clay => 0,
            Robot::Obsidian => 0,
            Robot::Geode => self.geode_robot_costs_obsidian,
        }
    }
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

#[derive(Debug, Clone)]
struct State {
    depth: u32,
    max_depth: u32,
    inventory: Inventory,
}

const fn div_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

impl State {
    fn new_with_one_ore(max_depth: u32) -> Self {
        State {
            depth: 0,
            max_depth,
            inventory: Inventory {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
        }
    }

    fn can_build_eventually(&self, blueprint: &Blueprint, robot: &Robot) -> Option<u32> {
        let missing_ore = blueprint
            .required_ore(robot)
            .saturating_sub(self.inventory.ore);
        let missing_clay = blueprint
            .required_clay(robot)
            .saturating_sub(self.inventory.clay);
        let missing_obsidian = blueprint
            .required_obsidian(robot)
            .saturating_sub(self.inventory.obsidian);

        let mut time = 0;

        if missing_ore > 0 {
            if self.inventory.ore_robots == 0 {
                return None;
            }
            time = time.max(div_up(missing_ore, self.inventory.ore_robots));
        }
        if missing_clay > 0 {
            if self.inventory.clay_robots == 0 {
                return None;
            }
            time = time.max(div_up(missing_clay, self.inventory.clay_robots));
        }
        if missing_obsidian > 0 {
            if self.inventory.obsidian_robots == 0 {
                return None;
            }
            time = time.max(div_up(missing_obsidian, self.inventory.obsidian_robots));
        }

        Some(time)
    }

    fn should_build(&self, blueprint: &Blueprint, robot: &Robot) -> bool {
        match robot {
            Robot::Ore => self.inventory.ore_robots < blueprint.max_costs_ore,
            Robot::Clay => self.inventory.clay_robots < blueprint.obsidian_robot_costs_clay,
            Robot::Obsidian => self.inventory.ore_robots < blueprint.geode_robot_costs_obsidian,
            Robot::Geode => true,
        }
    }

    fn advance_to_robot(&mut self, blueprint: &Blueprint, to_robot: &Robot, time: u32) {
        if self.depth + time >= self.max_depth {
            let remaining_time = self.max_depth - self.depth;
            self.inventory.ore += remaining_time * self.inventory.ore_robots;
            self.inventory.clay += remaining_time * self.inventory.clay_robots;
            self.inventory.obsidian += remaining_time * self.inventory.obsidian_robots;
            self.inventory.geodes += remaining_time * self.inventory.geode_robots;
            self.depth = self.max_depth;
        } else {
            self.inventory.ore += (time + 1) * self.inventory.ore_robots;
            self.inventory.clay += (time + 1) * self.inventory.clay_robots;
            self.inventory.obsidian += (time + 1) * self.inventory.obsidian_robots;
            self.inventory.geodes += (time + 1) * self.inventory.geode_robots;

            match to_robot {
                Robot::Ore => {
                    self.inventory.ore_robots += 1;
                    self.inventory.ore -= blueprint.ore_robot_costs_ore
                }
                Robot::Clay => {
                    self.inventory.clay_robots += 1;
                    self.inventory.ore -= blueprint.clay_robot_costs_ore
                }
                Robot::Obsidian => {
                    self.inventory.obsidian_robots += 1;
                    self.inventory.ore -= blueprint.obsidian_robot_costs_ore;
                    self.inventory.clay -= blueprint.obsidian_robot_costs_clay
                }
                Robot::Geode => {
                    self.inventory.geode_robots += 1;
                    self.inventory.ore -= blueprint.geode_robot_costs_ore;
                    self.inventory.obsidian -= blueprint.geode_robot_costs_obsidian
                }
            };

            self.depth += time + 1;
        }
    }

    fn next_states(&self, blueprint: &Blueprint) -> Vec<State> {
        if self.depth == self.max_depth {
            return Vec::new();
        }

        [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode]
            .iter()
            .filter(|robot| self.should_build(blueprint, robot))
            .filter_map(|robot| {
                let time = self.can_build_eventually(blueprint, robot)?;
                let mut state = self.clone();
                state.advance_to_robot(blueprint, robot, time);
                Some(state)
            })
            .collect::<Vec<State>>()
    }

    fn dfs(&self, blueprint: &Blueprint) -> u32 {
        let next_states = self.next_states(blueprint);

        if next_states.is_empty() {
            self.inventory.geodes
        } else {
            next_states
                .iter()
                .map(|state| state.dfs(blueprint))
                .max()
                .unwrap()
        }
    }
}

fn parse_blueprints(input: &str, max_blueprints: Option<usize>) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();

    input
        .trim_end()
        .lines()
        .take(max_blueprints.unwrap_or(usize::MAX))
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Blueprint {
                id: cap[1].parse().unwrap(),
                ore_robot_costs_ore: cap[2].parse().unwrap(),
                clay_robot_costs_ore: cap[3].parse().unwrap(),
                obsidian_robot_costs_ore: cap[4].parse().unwrap(),
                obsidian_robot_costs_clay: cap[5].parse().unwrap(),
                geode_robot_costs_ore: cap[6].parse().unwrap(),
                geode_robot_costs_obsidian: cap[7].parse().unwrap(),
                max_costs_ore: 0,
            }
        })
        .map(|blueprint: Blueprint| Blueprint {
            id: blueprint.id,
            ore_robot_costs_ore: blueprint.ore_robot_costs_ore,
            clay_robot_costs_ore: blueprint.clay_robot_costs_ore,
            obsidian_robot_costs_ore: blueprint.obsidian_robot_costs_ore,
            obsidian_robot_costs_clay: blueprint.obsidian_robot_costs_clay,
            geode_robot_costs_ore: blueprint.geode_robot_costs_ore,
            geode_robot_costs_obsidian: blueprint.geode_robot_costs_obsidian,
            max_costs_ore: blueprint.ore_robot_costs_ore.max(
                blueprint.clay_robot_costs_ore.max(
                    blueprint
                        .obsidian_robot_costs_ore
                        .max(blueprint.geode_robot_costs_ore),
                ),
            ),
        })
        .collect::<Vec<Blueprint>>()
}

fn part1(input: &str) -> usize {
    let blueprints = parse_blueprints(input, None);

    blueprints
        .iter()
        .map(|blueprint: &Blueprint| {
            blueprint.id as usize * State::new_with_one_ore(24).dfs(blueprint) as usize
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let blueprints = parse_blueprints(input, Some(3));

    blueprints
        .iter()
        .map(|blueprint: &Blueprint| State::new_with_one_ore(32).dfs(blueprint) as usize)
        .product()
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
        assert_eq!(part1(INPUT), 1266)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5800)
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
