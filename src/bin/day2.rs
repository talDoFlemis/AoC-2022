use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Play {
    Rock,
    Scissor,
    Paper,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Play::Rock && other == &Play::Scissor {
            Some(Ordering::Greater)
        } else if self == &Play::Scissor && other == &Play::Rock {
            Some(Ordering::Less)
        } else {
            Some(self.get_num().cmp(&other.get_num()))
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

    fn get_num(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn part1(play1: &Self, play2: &Self) -> u32 {
        let mut res: u32 = play1.get_num();
        res += match play1.partial_cmp(play2) {
            Some(Ordering::Greater) => 6,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Less) => 0,
            None => panic!("kkk"),
        };

        res
    }

    fn part2(opon: &Self, cond: &Self) -> u32 {
        match cond {
            //Loss condition
            Play::Rock => match &opon {
                Play::Rock => Play::Scissor.get_num(),
                Play::Paper => Play::Rock.get_num(),
                Play::Scissor => Play::Paper.get_num(),
            },
            //Draw condition
            Play::Paper => 3 + opon.get_num(),
            //Win condition
            Play::Scissor => {
                6 + match &opon {
                    Play::Rock => Play::Paper.get_num(),
                    Play::Paper => Play::Scissor.get_num(),
                    Play::Scissor => Play::Rock.get_num(),
                }
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
