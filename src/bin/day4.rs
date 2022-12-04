use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
struct ElfSections {
    start: u32,
    finish: u32,
}

impl ElfSections {
    fn new(input: &str) -> Self {
        let vec: Vec<u32> = input
            .split('-')
            .map(|e| e.parse::<u32>().unwrap())
            .collect();
        ElfSections {
            start: vec[0],
            finish: vec[1],
        }
    }
}

struct Pair {
    elf1: ElfSections,
    elf2: ElfSections,
}

impl Pair {
    fn new(input: &str) -> Self {
        let vec: Vec<ElfSections> = input.split(',').map(ElfSections::new).collect();
        Pair {
            elf1: vec[0],
            elf2: vec[1],
        }
    }

    fn overlap(&self) -> bool {
        let vec1: Vec<u32> = (self.elf1.start..=self.elf1.finish).collect();
        let vec2: Vec<u32> = (self.elf2.start..=self.elf2.finish).collect();
        let elf1_set: HashSet<u32> = (self.elf1.start..=self.elf1.finish).collect();
        let elf2_set: HashSet<u32> = (self.elf2.start..=self.elf2.finish).collect();
        if vec1.iter().all(|i| elf2_set.contains(i)) {
            true
        } else {
            vec2.iter().all(|i| elf1_set.contains(i))
        }
    }

    fn overlap_one(&self) -> bool {
        let vec1: Vec<u32> = (self.elf1.start..=self.elf1.finish).collect();
        let elf2_set: HashSet<u32> = (self.elf2.start..=self.elf2.finish).collect();
        vec1.iter().any(|i| elf2_set.contains(i))
    }
}
fn main() {
    let data = include_str!("../day4.txt");
    println!(
        "Overlaped results {}",
        data.lines().map(Pair::new).filter(|e| e.overlap()).count()
    );
    println!(
        "Overlaped at least one {}",
        data.lines()
            .map(Pair::new)
            .filter(|p| p.overlap_one())
            .count()
    );
}
