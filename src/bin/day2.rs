use rayon::prelude::*;
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Play::Rock, Play::Scissor) => Some(Ordering::Greater),
            (Play::Scissor, Play::Rock) => Some(Ordering::Less),
            _ => Some((*self as u32).cmp(&(*other as u32))),
        }
    }
}

impl Play {
    fn get_type(input: &char) -> Result<Self, ()> {
        match input {
            'X' | 'A' => Ok(Play::Rock),
            'Y' | 'B' => Ok(Play::Paper),
            'Z' | 'C' => Ok(Play::Scissor),
            _ => Err(()),
        }
    }

    fn part1(play1: &Self, play2: &Self) -> u32 {
        *play1 as u32
            + match play1.partial_cmp(play2) {
                Some(Ordering::Greater) => 6,
                Some(Ordering::Equal) => 3,
                Some(Ordering::Less) => 0,
                None => panic!("kkk"),
            }
    }

    fn part2(opon: &Self, cond: &Self) -> u32 {
        match cond {
            //Loss condition
            Play::Rock => {
                (match &opon {
                    Play::Rock => Play::Scissor,
                    Play::Paper => Play::Rock,
                    Play::Scissor => Play::Paper,
                }) as u32
            }
            //Draw condition
            Play::Paper => 3 + *opon as u32,
            //Win condition
            Play::Scissor => {
                6 + (match &opon {
                    Play::Rock => Play::Paper,
                    Play::Paper => Play::Scissor,
                    Play::Scissor => Play::Rock,
                }) as u32
            }
        }
    }

    pub fn run_game(input: &str) -> (u32, u32) {
        let args: Vec<char> = input.chars().collect();
        let (opo, me) = (
            Play::get_type(&args[0]).unwrap(),
            Play::get_type(&args[2]).unwrap(),
        );

        (Self::part1(&me, &opo), Self::part2(&opo, &me))
    }
}

fn part1(input: &str) -> u32 {
    match input {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

fn part2(input: &str) -> u32 {
    match input {
        "A X" => 3,
        "B X" => 1,
        "C X" => 2,
        "A Y" => 4,
        "B Y" => 5,
        "C Y" => 6,
        "A Z" => 8,
        "B Z" => 9,
        "C Z" => 7,
        _ => 0,
    }
}

fn part_2_nmeuleman(line: &str) -> u32 {
    // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
    let bytes = line.as_bytes();
    let left = (bytes[0] - b'A') as i8;
    let right = (bytes[2] - b'X') as i8;

    // 0 for rock, 1 for paper, 2 for scissors
    // 0 for loss, 1 for draw, 2 for win
    let opponent_shape = left;
    let outcome = right;
    let my_shape = (opponent_shape - 1 + outcome).rem_euclid(3);

    let shape_score = my_shape + 1;
    let outcome_score = 3 * outcome;
    (shape_score + outcome_score) as u32
}

fn part_1_nmeuleman(line: &str) -> u32 {
    // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
    let bytes = line.as_bytes();
    let left = (bytes[0] - b'A') as i8;
    let right = (bytes[2] - b'X') as i8;

    // 0 for rock, 1 for paper, 2 for scissors
    // 0 for loss, 1 for draw, 2 for win
    let opponent_shape = left;
    let my_shape1 = right;
    let outcome1 = (my_shape1 - opponent_shape + 1).rem_euclid(3);

    let shape_score1 = my_shape1 + 1;
    let outcome_score1 = 3 * outcome1;
    (shape_score1 + outcome_score1) as u32
}

fn polished_version(line: &str) -> (u32, u32) {
    let bytes = line.as_bytes();
    let left = (bytes[0] - b'A') as i8;
    let right = (bytes[2] - b'X') as i8;

    let opponent_shape = left;
    let my_shape1 = right;
    let outcome1 = (my_shape1 - opponent_shape + 1).rem_euclid(3);
    let outcome2 = right;
    let my_shape2 = (opponent_shape - 1 + outcome2).rem_euclid(3);

    let shape_score1 = my_shape1 + 1;
    let outcome_score1 = 3 * outcome1;

    let shape_score2 = my_shape2 + 1;
    let outcome_score2 = 3 * outcome2;
    (
        (shape_score1 + outcome_score1) as u32,
        (shape_score2 + outcome_score2) as u32,
    )
}

fn main() {
    let start = Instant::now();
    let data = include_str!("./input.txt");
    let mut score1 = 0;
    let mut score2 = 0;

    for line in data.lines() {
        let res = polished_version(line);
        score1 += res.0;
        score2 += res.1;
    }

    let finish = Instant::now();
    //500ms for NMEULEMAN part2
    //500ms for NMEULEMAN part2
    //800us for just part1 with match arms
    //800us for just part2 with match arms
    //2ms for creating structs
    //1ms running in match arms inside an unique iterator
    //1.5ms running in match arms inside an two iterators
    //2ms running with rayon with multiples iterators
    //1ms for NMEULEMAN solution with 2 iterators
    //600ms for NMEULEMAN solution with 2 iterators
    println!("My Score for part 1 is {score1}");
    println!("My Score for part 2 is {score2}");
    println!("Duration {:?}", finish.duration_since(start));
}
