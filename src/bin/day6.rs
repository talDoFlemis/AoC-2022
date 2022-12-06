use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let data: Vec<char> = input.chars().collect();
    let sequence = data
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set: HashSet<&char> = HashSet::from_iter(slice.iter());
            set.len() == slice.len()
        })
        .unwrap();

    sequence.0 + 4
}

fn part2(input: &str) -> usize {
    let data: Vec<char> = input.chars().collect();
    let sequence = data
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set: HashSet<&char> = HashSet::from_iter(slice.iter());
            set.len() == slice.len()
        })
        .unwrap();

    sequence.0 + 14
}
fn main() {
    let data = include_str!("../day6.txt");
    println!("Window of 4 is {}", part1(data));
    println!("Window of 14 is {}", part2(data));
}
