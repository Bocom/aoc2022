pub fn part_one(input: &str) -> Option<u32> {
    let mut current_acc: u32 = 0;
    let mut elves = Vec::<u32>::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_acc);
            current_acc = 0;
            continue;
        }

        current_acc += line.parse::<u32>().expect("Encountered invalid number");
    }
    elves.into_iter().max()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
