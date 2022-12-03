use std::{collections::HashSet, time::Instant};

fn get_intersection(a: &str, b: &str) -> String {
    let set1: HashSet<char> = HashSet::from_iter(a.chars());
    let set2: HashSet<char> = HashSet::from_iter(b.chars());
    let res = set1.intersection(&set2);
    String::from_iter(res)
}

fn get_common_item(input: &str) -> u32 {
    let size = input.len();
    let left = &input[..size / 2];
    let right = &input[size / 2..];

    let char = get_intersection(left, right).chars().next().unwrap();

    get_priority(char)
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u8 - 96).into()
    } else {
        (c as u8 - 38).into()
    }
}

fn get_badge(s1: &str, s2: &str, s3: &str) -> u32 {
    let common_s1_s2 = get_intersection(s1, s2);
    let common = get_intersection(&common_s1_s2, s3).chars().next().unwrap();

    get_priority(common)
}

fn part1(input: &str) -> u32 {
    input.lines().map(get_common_item).sum()
}

fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;
    for (idx, _) in lines.iter().enumerate().step_by(3) {
        sum += get_badge(lines[idx], lines[idx + 1], lines[idx + 2]);
    }

    sum
}

fn main() {
    let start = Instant::now();
    let data = include_str!("../day3.txt");
    let sum1 = part1(data);
    let sum2 = part2(data);
    let end = Instant::now();
    println!("The sum of priorities is {sum1}");
    println!("The sum of badges is {sum2}");
    println!("Duration is {:?}", end.duration_since(start));
}
