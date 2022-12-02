fn main() {
    let input = include_str!("./input.txt").split("\n\n");

    let mut tubias: Vec<u32> = input
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
