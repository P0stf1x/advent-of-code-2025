advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let ranges = input.split(",");
    for range in ranges {
        let mut range_iterator = range.split("-");
        let range_start = u64::from_str_radix(range_iterator.next().unwrap(), 10).unwrap();
        let range_end = u64::from_str_radix(range_iterator.next().unwrap(), 10).unwrap();

        for value in range_start..=range_end {
            let mut value_left_part = value.to_string();
            let value_len = value_left_part.len();
            if value_len % 2 != 0 {continue;} // some early optimisation
            let value_right_part = value_left_part.split_off(value_len/2);
            if value_left_part == value_right_part {
                result += value;
            }
        }
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let ranges = input.split(",");
    for range in ranges {
        let mut range_iterator = range.split("-");
        let range_start = u64::from_str_radix(range_iterator.next().unwrap(), 10).unwrap();
        let range_end = u64::from_str_radix(range_iterator.next().unwrap(), 10).unwrap();

        'value_generator: for value in range_start..=range_end {
            let value_str = value.to_string();
            let value_len = value_str.len();
            for split_size in 1..=value_len/2 {
                if value_len.rem_euclid(split_size) != 0 {continue;} // if we can't divide into whole chunks then just skip
                if value_str == value_str.split_at(split_size).0.repeat(value_len/split_size) {
                    result += value;
                    continue 'value_generator; // we added this value so no need to check other split_size'es
                }
            }
        }
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
