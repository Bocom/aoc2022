fn calculate_priority(c: char) -> u32 {
    let base: u8 = if c.is_lowercase() {
        b'a'
    } else {
        b'A'
    };
    let num = (c as u8 - base) as u32;

    if c.is_lowercase() {
        num + 1
    } else {
        num + 26 + 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut priority = 0u32;

    for line in input.lines() {
        let middle = line.len() / 2;
        let compartments = line.split_at(middle);
        let c1_chars = compartments.0.chars();
        let c2_chars = compartments.1.chars();

        let mut result: char = '0';
        'outer: for lch in c1_chars {
            for rch in c2_chars.clone() {
                if lch == rch {
                    result = lch;
                    break 'outer;
                }
            }
        }

        priority += calculate_priority(result);
    }

    Some(priority)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut priority = 0u32;

    let all_lines = _input.lines().collect::<Vec<_>>();
    for chunk in all_lines.chunks(3) {
        let bag1 = chunk[0].chars();
        let bag2 = chunk[1].chars();
        let bag3 = chunk[2].chars();

        let mut result: char = '0';
        'outer: for b1 in bag1 {
            for b2 in bag2.clone() {
                for b3 in bag3.clone() {
                    if b1 == b2 && b2 == b3 {
                        result = b1;
                        break 'outer;
                    }
                }
            }
        }

        priority += calculate_priority(result);
    }

    Some(priority)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
