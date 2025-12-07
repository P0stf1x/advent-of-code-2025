advent_of_code::solution!(7);

enum BoardTile {
    Beam(u64),
    Splitter,
    Air,
}

fn convert_input_to_board(input: &str) -> Vec<Vec<BoardTile>> {
    input.lines().map(|line|{
        line.chars().map(|symbol|{
            match symbol {
                '.' => BoardTile::Air,
                '^' => BoardTile::Splitter,
                'S' => BoardTile::Beam(1),
                 c  => panic!("Something unexpected on the board: '{}'", c),
            }
        }).collect()
    }).collect()
}

struct BoardResult {
    timelines: u64,
    splitters: u64,
}

fn solve_board(board: &mut Vec<Vec<BoardTile>>) -> BoardResult {
    let mut splitters = 0u64;
    let bottom_edge = board.len();
    let right_edge = board[0].len();
    for row in 0..board.len() {
        for column in 0..board[row].len() {
            match board[row][column] {
                BoardTile::Air => (),
                BoardTile::Splitter => (),
                BoardTile::Beam(value) => {
                    if row + 1 >= bottom_edge {continue;}
                    match board[row+1][column] {
                        BoardTile::Air => board[row+1][column] = BoardTile::Beam(value),
                        BoardTile::Beam(other_value) => {
                            board[row+1][column] = BoardTile::Beam(value+other_value);
                        },
                        BoardTile::Splitter => {
                            splitters += 1;
                            #[allow(unused_comparisons)] // that's easier to understand for my brain rather than remembering that the type is unsigned
                            if column-1 >= 0 {
                                let other_value = match board[row+1][column-1] {
                                    BoardTile::Beam(value) => value,
                                    _ => 0,
                                };
                                board[row+1][column-1] = BoardTile::Beam(value+other_value);
                            };
                            if column+1 < right_edge {
                                let other_value = match board[row+1][column+1] {
                                    BoardTile::Beam(value) => value, // actually unneeded since we go left to right
                                    _ => 0,
                                };
                                board[row+1][column+1] = BoardTile::Beam(value+other_value);
                            };
                            // potential FIXME: if there are two splitters next to each other they would overwrite each other
                        }
                    }
                }
            }
        }
    };
    let timelines: u64 = board[board.len()-1].iter()
        .map(|tile|{
            match tile {
                BoardTile::Beam(val) => val.clone(),
                _ => 0,
            }
        })
        .sum();
    return BoardResult { timelines, splitters };
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut board = convert_input_to_board(input);
    let result = solve_board(&mut board);
    return Some(result.splitters);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut board = convert_input_to_board(input);
    let result = solve_board(&mut board);
    return Some(result.timelines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
