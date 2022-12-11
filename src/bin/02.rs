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
            _ => Err(InvalidMove {})
        }
    }
}

enum RoundResult {
    Win,
    Loss,
    Draw
}

fn round_result(opponent_move: Move, predicted_move: Move) -> RoundResult {
    if opponent_move == predicted_move {
        return RoundResult::Draw;
    } else if opponent_move == Move::Rock {
        if predicted_move == Move::Paper {
            RoundResult::Win
        } else {
            RoundResult::Loss
        }
    } else if opponent_move == Move::Paper {
        if predicted_move == Move::Rock {
            RoundResult::Loss
        } else {
            RoundResult::Win
        }
    } else {
        if predicted_move == Move::Rock {
            RoundResult::Win
        } else {
            RoundResult::Loss
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0u32;

    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let opponent_move = Move::try_from(split[0]).expect("Invalid opponent move");
        let predicted_move = Move::try_from(split[1]).expect("Invalid prediciton");

        let round_result = round_result(opponent_move.clone(), predicted_move.clone());

        score += match round_result {
            RoundResult::Win => 6,
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
        };

        score += match predicted_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
    }

    Some(score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
