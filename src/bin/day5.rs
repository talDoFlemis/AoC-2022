use std::collections::VecDeque;

use regex::Regex;

struct StackDusGuri {
    items: Vec<char>,
}

struct Instructions {
    quant: usize,
    from: usize,
    to: usize,
}

impl StackDusGuri {
    fn new(input: &str, off: usize) -> Self {
        let mut v: Vec<char> = Vec::new();
        for (i, line) in input.lines().rev().enumerate() {
            let c = line.chars().nth(off).unwrap();
            if i == 0 {
                continue;
            } else if !c.is_whitespace() {
                v.push(c);
            }
        }

        StackDusGuri { items: v }
    }
}

struct SwitchMEDADDY {
    stacks: Vec<StackDusGuri>,
    ins: Vec<Instructions>,
}

impl SwitchMEDADDY {
    fn new(input: &str, size: usize) -> Self {
        let mut off = 1;
        let data: Vec<&str> = input.split("\n\n").collect();

        let mut stacks: Vec<StackDusGuri> = Vec::new();
        let mut ins: Vec<Instructions> = Vec::new();
        (0..size).for_each(|_| {
            stacks.push(StackDusGuri::new(data[0], off));
            off += 4;
        });

        let re = Regex::new(r"move (?P<q>\d{1,2}) from (?P<f>\d) to (?P<t>\d)").unwrap();
        for line in data[1].lines() {
            let cap = re.captures(line).unwrap();
            ins.push(Instructions {
                quant: cap["q"].parse::<usize>().unwrap(),
                from: cap["f"].parse::<usize>().unwrap(),
                to: cap["t"].parse::<usize>().unwrap(),
            })
        }

        SwitchMEDADDY { stacks, ins }
    }
    fn part1(&mut self) -> String {
        for ins in self.ins.iter() {
            for _ in 0..ins.quant {
                let out = self.stacks[ins.from - 1].items.pop().unwrap();
                self.stacks[ins.to - 1].items.push(out);
            }
        }

        String::from_iter(self.stacks.iter().map(|s| s.items.last().unwrap()))
    }

    fn part2(&mut self) -> String {
        for ins in self.ins.iter() {
            let mut v: VecDeque<char> = VecDeque::new();
            for _ in 0..ins.quant {
                let out = self.stacks[ins.from - 1].items.pop().unwrap();
                v.push_front(out);
            }

            for _ in 0..ins.quant {
                let out = v.pop_front().unwrap();
                self.stacks[ins.to - 1].items.push(out);
            }
        }

        String::from_iter(self.stacks.iter().map(|s| s.items.last().unwrap()))
    }
}

fn main() {
    let data = include_str!("../day5.txt");
    let mut game = SwitchMEDADDY::new(data, 9);
    println!("Result is {}", SwitchMEDADDY::part1(&mut game));
    let mut game2 = SwitchMEDADDY::new(data, 9);
    println!("Result is {}", SwitchMEDADDY::part2(&mut game2));
}
