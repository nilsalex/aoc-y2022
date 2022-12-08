#![feature(test)]
#![feature(iter_intersperse)]
extern crate test;

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn total_size(dir_name: &str, size_map: &HashMap<String, usize>, dir_map: &HashMap<String, Vec<String>>) -> usize {
    let mut result = *size_map.get(dir_name).unwrap_or(&0);

    if let Some(subdirs) = dir_map.get(dir_name) {
        for subdir in subdirs {
            let subsize = total_size(subdir, size_map, dir_map);
            result += subsize;
        }
    }

    result
}

fn dir_name(vec: &[String]) -> String {
    let mut result = String::new();
    for str in vec.iter().intersperse(&String::from("/")) {
        result.push_str(str);
    }
    result
}

fn part1(input: &str) -> usize {
    let mut size_map: HashMap<String, usize> = HashMap::new();
    let mut dir_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cur_dir: Vec<String> = Vec::new();
    let mut all_dirs: Vec<String> = Vec::new();

    for line in input.lines() {
        if line == "$ cd .." {
            cur_dir.pop();
        }
        else if line.starts_with("$ cd") {
            cur_dir.push(String::from(&line[5..]));
            let cur_dir_name = dir_name(&cur_dir);
            all_dirs.push(cur_dir_name)
        } else if line.starts_with("dir ") {
            let cur_dir_name = dir_name(&cur_dir);
            cur_dir.push(String::from(&line[4..]));
            let new_dir_name = dir_name(&cur_dir);
            cur_dir.pop();
            if let Some(list) = dir_map.get_mut(&cur_dir_name) {
                list.push(new_dir_name);
            } else {
                dir_map.insert(cur_dir_name, vec![new_dir_name]);
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let space_pos = line.find(' ').unwrap();
            let cur_dir_name = dir_name(&cur_dir);
            let size = &line[0..space_pos].parse::<usize>().unwrap();
            if let Some(total_size) = size_map.get_mut(&cur_dir_name) {
                *total_size += *size;
            } else {
                size_map.insert(cur_dir_name, *size);
            }
        }
    };

    let mut result = 0;

    for dir in all_dirs {
        let size = total_size(&dir, &size_map, &dir_map);
        if size <= 100000 {
            result += size;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let mut size_map: HashMap<String, usize> = HashMap::new();
    let mut dir_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cur_dir: Vec<String> = Vec::new();
    let mut all_dirs: Vec<String> = Vec::new();

    for line in input.lines() {
        if line == "$ cd .." {
            cur_dir.pop();
        }
        else if line.starts_with("$ cd") {
            cur_dir.push(String::from(&line[5..]));
            let cur_dir_name = dir_name(&cur_dir);
            all_dirs.push(cur_dir_name)
        } else if line.starts_with("dir ") {
            let cur_dir_name = dir_name(&cur_dir);
            cur_dir.push(String::from(&line[4..]));
            let new_dir_name = dir_name(&cur_dir);
            cur_dir.pop();
            if let Some(list) = dir_map.get_mut(&cur_dir_name) {
                list.push(new_dir_name);
            } else {
                dir_map.insert(cur_dir_name, vec![new_dir_name]);
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let space_pos = line.find(' ').unwrap();
            let cur_dir_name = dir_name(&cur_dir);
            let size = &line[0..space_pos].parse::<usize>().unwrap();
            if let Some(total_size) = size_map.get_mut(&cur_dir_name) {
                *total_size += *size;
            } else {
                size_map.insert(cur_dir_name, *size);
            }
        }
    };

    let free_space = 70000000 - total_size("/", &size_map, &dir_map) as isize;
    let to_free = 30000000 - free_space;

    all_dirs
        .iter()
        .map(|dir| {
        let dir_size = total_size(dir, &size_map, &dir_map) as isize;
        (dir_size - to_free, dir_size)
    })
        .filter(|(diff, _)| *diff >= 0)
        .min_by(|(diff1, _), (diff2, _)| diff1.cmp(diff2))
        .unwrap()
        .1 as usize
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
        assert_eq!(part1(INPUT), 919137)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2877389)
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
