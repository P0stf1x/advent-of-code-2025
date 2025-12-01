use std::ops::Div;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut zeroes = 0;
    let mut position = 50;
    for line in input.lines() {
        let offset =
            {if line.starts_with("L") {-1} else {1}} // negative if starts with L
            * i32::from_str_radix(&line[1..], 10).unwrap(); // value in the line

        position = (position + offset).rem_euclid(100);
        if position == 0 {zeroes += 1};
    };
    return Some(zeroes);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut zeroes = 0;
    let mut position = 50;
    for line in input.lines() {
        let offset = i32::from_str_radix(&line[1..], 10).unwrap();
        let negative = line.starts_with("L");

        let diff_to_zero = {
            if negative {
                position
            } else {
                100 - position
            }
        };

        if offset >= diff_to_zero {
            let remaining_offset = offset - diff_to_zero;
            if !negative || position != 0 { // if we're going down and we're currently at zero then we shouldn't tick that time (i know this is janky but math is hard)
                zeroes += 1
            }
            let zeroes_to_tick = (remaining_offset / 100) as u64;
            zeroes += zeroes_to_tick;
        }

        if negative {
            position = (position - offset).rem_euclid(100);
        } else {
            position = (position + offset).rem_euclid(100);
        }
    };
    return Some(zeroes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
