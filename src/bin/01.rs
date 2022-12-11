fn count_calories(input: &str) -> Vec<u32> {
    let mut elves = Vec::<u32>::new();
    let mut current_acc: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_acc);
            current_acc = 0;
            continue;
        }

        current_acc += line.parse::<u32>().expect("Encountered invalid number");
    }

    if current_acc != 0 {
        elves.push(current_acc);
    }

    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = count_calories(input);
    elves.into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = count_calories(input);
    elves.sort();
    let top_three = elves
        .iter()
        .rev()
        .take(3);

    Some(top_three.sum())
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
        assert_eq!(part_two(&input), Some(45_000));
    }
}
