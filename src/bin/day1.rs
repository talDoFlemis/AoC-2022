fn main() {
    let input: Vec<&str> = include_str!("./input.txt").split("\n\n").collect();

    let mut tubias: Vec<u32> = input
        .iter()
        .map(|e| {
            e.split('\n')
                .filter_map(|num| num.parse::<u32>().ok())
                .sum()
        })
        .collect();

    tubias.sort_by(|a, b| b.cmp(a));

    println!("{:?}", tubias[0]);
    println!("{}", tubias.iter().take(3).sum::<u32>());
}
