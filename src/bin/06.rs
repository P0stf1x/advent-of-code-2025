advent_of_code::solution!(6);

fn transpose<T>(vec2d: &Vec<Vec<T>>) -> Vec<Vec<T>>
where T: Copy {
    // Transforms matrix [[A, B], into [[A, C],
    //                    [C, D]]       [B, D]]
    (0..vec2d[0].len()).map(|x|{
        vec2d.iter().map(|row|{row[x]}).collect()
    }).collect()
}

fn what_the_actual_fuck_is_this(str_values: Vec<String>) -> Vec<Vec<u64>> {
    /*
    I don't fucking know what the actual fuck is this ugly abomination of a code but I can somewhat explain what it does:
    So, basically we have a vector of strings, something like ["123", "321", "", "456", "654"]
    And we split it by whitespace strings, so we get [["123", "321"], ["456", "654"]]
    But since we need them as u64 instead of actual string, we also transform it there. [[123, 321], [456, 654]]

    also I'm not making it generic, that's on you
    */
    str_values.split(|line|{line.trim().is_empty()}) // split produces iterator of &[T]
        .map(|borrowed_array_of_str|{borrowed_array_of_str.to_vec().iter().map(|actual_string|{ // So we can either collect right away into Vec<&[T]>, or we can to_vec() each &[T] so we get Vec<Vec<T>>
            u64::from_str_radix(actual_string.trim(), 10).unwrap() // On this line we convert T from String into u64
        }).collect()}) // And collect it into Vec<u64>
        .collect() // And once again collect it into Vec<Vec<u64>>
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut inputs_vec: Vec<Vec<&str>> = input.lines().map(|line|{
        line.split_whitespace().collect()
    }).collect();
    let operations = inputs_vec.pop().unwrap();
    let values: Vec<Vec<u64>> = inputs_vec.iter().map(|line|{
        line.iter().map(|value|{
            u64::from_str_radix(value, 10).unwrap()
        }).collect()
    }).collect();
    let better_positioned_values = transpose(&values);

    let mut result = 0u64;
    for column in 0..better_positioned_values.len() {
        result += match operations[column] {
            "+" => better_positioned_values[column].iter().copied().reduce(|a, b|{a + b}).unwrap(),
            "*" => better_positioned_values[column].iter().copied().reduce(|a, b|{a * b}).unwrap(),
             _  => panic!("How did we get here"),
        };
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut values: Vec<Vec<char>> = input.lines().map(|line|{line.chars().collect()}).collect();
    let operations_temp: String = values.pop().unwrap().iter().collect();
    let operations: Vec<&str> = operations_temp.split_whitespace().collect();
    values = transpose(&values);
    let str_values: Vec<String> = values.iter().map(|line_vec|{line_vec.iter().collect()}).collect();
    let values_as_vectors_to_be_added_or_something: Vec<Vec<u64>> = what_the_actual_fuck_is_this(str_values);

    let mut result = 0u64;
    for column in 0..values_as_vectors_to_be_added_or_something.len() {
        result += match operations[column] {
            "+" => values_as_vectors_to_be_added_or_something[column].iter().copied().reduce(|a, b|{a + b}).unwrap(),
            "*" => values_as_vectors_to_be_added_or_something[column].iter().copied().reduce(|a, b|{a * b}).unwrap(),
             _  => panic!("How did we get here"),
        };
    }

    // it would've been easier to just make an interpreter, rather than transforming it into usable state
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
