use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct DusGuri {
    name: String,
    size: u32,
}

impl DusGuri {
    fn new(name: &str, size: u32) -> Self {
        let name = String::from(name);
        let size = size;
        DusGuri { name, size }
    }
}

fn part1(input: &str) -> u32 {
    let mut stack: Vec<(&str, u32)> = Vec::new();
    let mut tubias: HashSet<DusGuri> = HashSet::new();
    let max = 100000;

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.as_slice() {
            ["$", "cd", ".."] => {
                let node = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += node.1;
                if node.1 <= max {
                    tubias.insert(DusGuri::new(node.0, node.1));
                }
            }
            ["$", "cd", dir] => {
                stack.push((dir, 0));
            }
            [number, _] => {
                stack.last_mut().unwrap().1 += number.parse::<u32>().unwrap();
            }
            _ => println!("rubias"),
        }
    }
    tubias.iter().map(|k| k.size).sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let mut stack: Vec<(&str, u32)> = Vec::new();
    let mut tubias: HashSet<DusGuri> = HashSet::new();
    let tot_space = 70000000;

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.as_slice() {
            ["$", "cd", ".."] => {
                let node = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += node.1;
                tubias.insert(DusGuri::new(node.0, node.1));
            }
            ["$", "cd", dir] => {
                stack.push((dir, 0));
            }
            [number, _] => {
                stack.last_mut().unwrap().1 += number.parse::<u32>().unwrap();
            }
            _ => println!("rubias"),
        }
    }

    let unused_space = tot_space - stack.iter().map(|e| e.1).sum::<u32>();
    let space_needed = 30000000 - unused_space;

    tubias
        .iter()
        .map(|k| k.size)
        .filter(|e| e >= &space_needed)
        .min()
        .unwrap()
}

fn main() {
    let data = include_str!("../day7.txt");
    println!("Result of part 1 is {}", part1(data));
    println!("Result of part 2 is {}", part2(data));
}
