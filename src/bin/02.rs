use std::convert::TryFrom;

#[derive(PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
struct InvalidMove;

impl TryFrom<&str> for Move {
    type Error = InvalidMove;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(Self::Error {})
        }
    }
}

enum RoundResult {
    Win,
    Loss,
    Draw
}

#[derive(Debug)]
struct InvalidResult;

impl TryFrom<&str> for RoundResult {
    type Error = InvalidResult;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(Self::Error {})
        }
    }
}

fn calculate_score(used_move: Move, round_result: RoundResult) -> u32 {
    let mut score = 0;

    score += match round_result {
        RoundResult::Win => 6,
        RoundResult::Loss => 0,
        RoundResult::Draw => 3,
    };

    score += match used_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0u32;

    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let opponent_move = Move::try_from(split[0]).expect("Invalid opponent move");
        let predicted_move = Move::try_from(split[1]).expect("Invalid prediciton");

        let round_result = match opponent_move {
            Move::Rock => match predicted_move {
                Move::Rock => RoundResult::Draw,
                Move::Paper => RoundResult::Win,
                Move::Scissors => RoundResult::Loss,
            },
            Move::Paper => match predicted_move {
                Move::Rock => RoundResult::Loss,
                Move::Paper => RoundResult::Draw,
                Move::Scissors => RoundResult::Win,
            },
            Move::Scissors => match predicted_move {
                Move::Rock => RoundResult::Win,
                Move::Paper => RoundResult::Loss,
                Move::Scissors => RoundResult::Draw,
            },
        };

        score += calculate_score(predicted_move, round_result);
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0u32;

    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let opponent_move = Move::try_from(split[0]).expect("Invalid opponent move");
        let expected_result = RoundResult::try_from(split[1]).expect("Invalid expected result");

        let predicted_move = match expected_result {
            RoundResult::Win => match opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            RoundResult::Loss => match opponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            RoundResult::Draw => match opponent_move {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
        };

        score += calculate_score(predicted_move, expected_result);
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
