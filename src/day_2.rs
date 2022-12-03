use crate::helpers;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Moves {
    Unset = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Result {
    Unset = -1,
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Clone, Copy)]
struct Round {
    opponent: Moves,
    player: Moves,
    result: Result,
}

impl Round {
    fn get_result(&self) -> Result {
        if self.result != Result::Unset {
            return self.result;
        }

        if self.opponent == Moves::Rock && self.player == Moves::Scissors {
            return Result::Loss;
        }

        if self.player == Moves::Rock && self.opponent == Moves::Scissors {
            return Result::Win;
        }

        if self.opponent > self.player {
            return Result::Loss;
        }

        if self.player > self.opponent {
            return Result::Win;
        }

        Result::Draw
    }

    fn set_player_move(&mut self) {
        match self.result {
            Result::Draw => self.player = self.opponent,
            Result::Win => {
                match self.opponent {
                    Moves::Rock => self.player = Moves::Paper,
                    Moves::Paper => self.player = Moves::Scissors,
                    Moves::Scissors => self.player = Moves::Rock,
                    Moves::Unset => panic!("Opponent's move is unset"),
                };
            },
            Result::Loss => {
                match self.opponent {
                    Moves::Rock => self.player = Moves::Scissors,
                    Moves::Paper => self.player = Moves::Rock,
                    Moves::Scissors => self.player = Moves::Paper,
                    Moves::Unset => panic!("Opponent's move is unset"),
                };
            },
            Result::Unset => panic!("Result is not set"),
        }
    }

    fn get_score(&self) -> u32 {
        let result = self.get_result();

        (result as u32) + (self.player as u32)
    }
}

fn get_rounds(part: u8) -> Vec<Round> {
    let inputs = helpers::read_inputs_file(2);

    let mut rounds: Vec<Round> = vec![];

    for line in inputs {
        let mut round = Round {
            opponent: Moves::Unset,
            player: Moves::Unset,
            result: Result::Unset,
        };

        let opponent = line.as_bytes()[0] as char;
        let player = line.as_bytes()[2] as char;

        match opponent {
            'A' => round.opponent = Moves::Rock,
            'B' => round.opponent = Moves::Paper,
            'C' => round.opponent = Moves::Scissors,
            _ => panic!("Invalid input"),
        };

        match player {
            'X' => {
                match part {
                    1 => round.player = Moves::Rock,
                    2 => round.result = Result::Loss,
                    _ => panic!("Invalied part number"),
                }
            },
            'Y' => {
                match part {
                    1 => round.player = Moves::Paper,
                    2 => round.result = Result::Draw,
                    _ => panic!("Invalied part number"),
                }
            },
            'Z' => {
                match part {
                    1 => round.player = Moves::Scissors,
                    2 => round.result = Result::Win,
                    _ => panic!("Invalied part number"),
                }
            },
            _ => panic!("Invalid input"),
        };

        rounds.push(round);
    }

    rounds
}

pub fn solve_part(part: u8) -> u32 {
    let mut rounds = get_rounds(part);

    if part == 2 {
        rounds.iter_mut().for_each(|round| round.set_player_move());
    }

    rounds.iter().map(|round| round.get_score()).sum()
}
