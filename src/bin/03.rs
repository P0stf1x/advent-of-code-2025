advent_of_code::solution!(3);

fn generalized_solution(input: &str, battery_count: usize) -> u64 {
    let mut result = 0u64;
    for line in input.lines() {
        let mut line_result = 0u64;
        let mut highest_number_search_position = 0;

        for digits_place in (0..battery_count).rev() {
            let mut highest_number = 0;
            let mut highest_number_position = 0;
            let end_search_position = line.len() - digits_place;
            //  ^ so that when we look for highest digit in tens place we would leave at least one digit for ones place digit
            //    e.g. when we are looking for tens place if we use final 9 (v) it would leave no space for ones, and it's would be out of bounds
            //                                        example input: 123456789

            for (position, digit) in (line[highest_number_search_position..end_search_position]).chars().enumerate() {
                let actual_position = position + highest_number_search_position; // since enumerate starts at 0, but we start at 0+search_pos
                let digit_int = u64::from_str_radix(digit.to_string().as_str(), 10).unwrap();

                if digit_int > highest_number {
                    highest_number = digit_int;
                    highest_number_position = actual_position;
                }
            }
            highest_number_search_position = highest_number_position+1; // so that next places digit starts from that position
            line_result += highest_number * 10u64.pow(digits_place as u32);
        }
        result += line_result;
    }
    return result;

}

pub fn part_one(input: &str) -> Option<u64> {
    Some(generalized_solution(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(generalized_solution(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
