use std::collections::HashSet;

struct Rucksack {
    data: String,
    shared_item: u32,
}

impl Rucksack {
    fn new(input: &str) -> Self {
        let size = input.len();
        let left = &input[0..size / 2];
        let right = &input[size / 2..size];
        let set1: HashSet<char> = right.chars().collect();
        let common = left.chars().find(|c| set1.contains(c)).unwrap();

        Rucksack {
            data: String::from(input),
            shared_item: Self::get_priority(common),
        }
    }

    fn get_priority(c: char) -> u32 {
        if c.is_lowercase() {
            (c as u8 - 96).into()
        } else {
            (c as u8 - 38).into()
        }
    }
}

struct Group {
    common_badge: u32,
}

impl Group {
    fn new(input: &str) -> Self {
        let vec: Vec<Rucksack> = input.lines().map(Rucksack::new).take(3).collect();
        let set1: HashSet<char> = vec[1].data.chars().collect();
        let set2: HashSet<char> = vec[2].data.chars().collect();
        let common = vec[0]
            .data
            .chars()
            .filter(|c| set1.contains(c))
            .collect::<String>();
        let common = common.chars().find(|c| set2.contains(c)).unwrap();
        println!("Common is {common:?}");

        Group {
            common_badge: Rucksack::get_priority(common),
        }
    }
}

fn part1(input: &str) -> u32 {
    input.lines().map(|l| Rucksack::new(l).shared_item).sum()
}

fn part2(input: &str) -> u32 {
    let mut counter = 0;
    let mut sum = 0;
    let mut string = String::new();
    for line in input.lines() {
        if counter == 2 {
            string.push_str(line);
            string.push('\n');
            println!("Group is {string}");

            sum += Group::new(&string).common_badge;
            string = "".to_string();
            counter = 0;
        } else {
            string.push_str(line);
            string.push('\n');
            println!("String is {string}");
            counter += 1;
        }
    }

    sum
}

fn main() {
    let data = include_str!("../day3.txt");
    let sum1 = part1(data);
    let sum2 = part2(data);
    println!("The sum of priorities is {sum1}");
    println!("The sum of badges is {sum2}");
}
