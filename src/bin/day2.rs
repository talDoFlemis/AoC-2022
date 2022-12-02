use std::cmp::Ordering;

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

fn main() {
    let data = include_str!("./input.txt");
    let mut score1 = 0;
    let mut score2 = 0;

    for line in data.lines() {
        let res = Play::run_game(line);
        score1 += res.0;
        score2 += res.1;
    }

    println!("My Score for part 1 is {score1}");
    println!("My Score for part 2 is {score2}");
}
