advent_of_code::solution!(4);

struct RemovalResult {
    new_state: Vec<Vec<bool>>,
    removed_rolls: u64,
}

fn remove_rolls(state: Vec<Vec<bool>>) -> RemovalResult {
    let mut new_state = state.clone();
    let mut removed_rolls = 0u64;
    let y_len = state.len() as isize;
    let x_len = state[0].len() as isize;
    for y in 0isize..y_len {
        for x in 0isize..x_len {
            if !state[x as usize][y as usize] {continue;}
            let mut neighbors = 0;
            for y_offset in -1..=1isize {
                for x_offset in -1..=1isize {
                    if x_offset == 0 && y_offset == 0 {continue;} // don't count itself
                    let checked_x = x+x_offset;
                    let checked_y = y+y_offset;
                    if checked_x < 0 || checked_y < 0 || checked_x >= x_len || checked_y >= y_len {continue;}
                    if state[checked_x as usize][checked_y as usize] {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                removed_rolls += 1;
                new_state[x as usize][y as usize] = false;
            }
        }
    };
    return RemovalResult { new_state, removed_rolls };
}

pub fn part_one(input: &str) -> Option<u64> {
    let rolls: Vec<Vec<bool>> = input.lines().map(|line| {
        line.chars().map(|char| {
            match char {
                '@' => true,
                 _  => false,
            }
        }).collect()
    }).collect();

    let removal_result = remove_rolls(rolls);
    return Some(removal_result.removed_rolls);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0u64;
    let mut state: Vec<Vec<bool>> = input.lines().map(|line| {
        line.chars().map(|char| {
            match char {
                '@' => true,
                 _  => false,
            }
        }).collect()
    }).collect();

    loop {
        let removal_result = remove_rolls(state);
        state = removal_result.new_state;
        result += removal_result.removed_rolls;
        if removal_result.removed_rolls <= 0 {break;}
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
