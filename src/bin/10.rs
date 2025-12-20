advent_of_code::solution!(10);

#[derive(Clone, Copy)]
struct Lights(u64); // We can represent lights as u64 where each bit is a single indicator

impl Lights {
    fn from_light_diagram(diagram: &str) -> Self {
        let bits: Vec<char> = diagram[1..diagram.len()-1].chars().collect();
        let mut result = 0u64;
        for i in 0..bits.len() {
            result ^= match bits[i] {
                '#' => 1u64 << i,
                 _  => 0,
            }
        };
        return Self(result);
    }

    fn from_toggle(bits_str: &str) -> Self {
        let bits: Vec<usize> = bits_str[1..bits_str.len()-1].split(",").map(|bit|{
            usize::from_str_radix(bit, 10).unwrap()
        }).collect();
        let mut result = 0u64;
        for bit in bits {
            result ^= 1u64 << bit
        };
        return Self(result);
    }
}

struct TogglesStorage {
    toggles: Vec<Lights>,
}

impl TogglesStorage {
    fn iterate_combinations(&self, check: &Lights) -> u64 {
        let mut smallest_valid = usize::MAX;
        let toggles_total = self.toggles.len();
        'next_combination: for bitmask in 1..(1 << toggles_total) { // We iterate through all (except all off) combinations of 2^toggles_total bits
            let mut toggles_combination: Vec<Lights> = Vec::with_capacity(toggles_total);
            for i in 0..toggles_total {
                if (bitmask & (1 << i)) != 0 { // If i'th bit is 1 we push toggles[i] into vector
                    toggles_combination.push(self.toggles[i]);
                }
            }
            // We don't need to even check whether they match if there's more toggles than current minimum
            let combination_length = toggles_combination.len();
            if combination_length >= smallest_valid {continue 'next_combination;}

            let xor_ed = toggles_combination.into_iter().reduce(|light_a, light_b|{
                Lights(light_a.0 ^ light_b.0)
            }).unwrap();
            if xor_ed.0 == check.0 {smallest_valid = combination_length}
            if smallest_valid == 1 {return 1u64;} // we can return 1 right away since there's no empty lights
        };
        return smallest_valid as u64;
    }

    fn new(toggles: Vec<Lights>) -> Self {
        Self { toggles }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines_split: Vec<Vec<&str>> = input.lines().map(|line|{
        line.split_whitespace().collect()
    }).collect();
    let lights: Vec<Lights> = lines_split.iter().map(|line|{
        Lights::from_light_diagram(line[0])
    }).collect();
    let toggles: Vec<Vec<Lights>> = lines_split.iter().map(|line|{
        line[1..line.len()-1].iter().map(|bits_str|{
            Lights::from_toggle(bits_str)
        }).collect()
    }).collect();
    let mut result = 0u64;
    for i in 0..lights.len() {
        let ts = TogglesStorage::new(toggles[i].clone());
        result += ts.iterate_combinations(&lights[i]);
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines_split: Vec<Vec<&str>> = input.lines().map(|line|{
        line.split_whitespace().collect()
    }).collect();

    let results: Vec<Vec<f64>> = lines_split.iter().map(|line|{
        line[line.len()-1][1..line[line.len()-1].len()-1].split(",").map(|val|{
            u64::from_str_radix(val, 10).unwrap() as f64
        }).collect()
    }).collect();

    let toggles_pre_matrix_conversion: Vec<Vec<Vec<f64>>> = lines_split.iter().map(|line|{
        line[1..line.len()-1].iter().map(|toggle_str|{
            toggle_str[1..toggle_str.len()-1].split(",").map(|val|{
                u64::from_str_radix(val, 10).unwrap() as f64
            }).collect()
        }).collect()
    }).collect();

    let toggles: Vec<Vec<Vec<f64>>> = (0..results.len()).map(|i|{
        (0..results[i].len()).map(|y|{
            toggles_pre_matrix_conversion[i].iter().map(|line|{
                if line.contains(&(y as f64)) {1.} else {0.}
            }).collect()
        }).collect()
    }).collect();

    let mut result = 0u64;
    for i in 0..toggles.len() {
        let (_, presses, free_vectors) = gauss_jordan(&toggles[i], &results[i]);

        if free_vectors.len() == 0 {
            result += presses.0.iter().sum::<f64>() as u64;
            continue;
        }

        let max_iters = *(results[i].iter().max_by_key(|v|{v.round().abs() as u64}).unwrap()) as usize;
        let mut vectors_iterator = NIncrementor::new(free_vectors.len(), max_iters);
        let mut minimum = f64::INFINITY;

        for vector_multiplier in vectors_iterator.iter_mut() {
            let free_result = vector_multiplier.iter().sum::<usize>();
            if free_result as f64 >= minimum {continue;}

            let mut buttons_result = presses.clone();
            for vector_i in 0..free_vectors.len() {
                buttons_result = &buttons_result - &(&free_vectors[vector_i] * vector_multiplier[vector_i])
            }
            if buttons_result.is_negative() {continue;}
            if !buttons_result.is_whole() {continue;}
            let buttons_result_sum = buttons_result.0.iter().sum::<f64>();

            if ((free_result as f64) + buttons_result_sum) < minimum {
                minimum = (free_result as f64) + buttons_result_sum;
            }
        }
        result += (minimum+0.1).round() as u64;
    }
    return Some(result);
}

struct NIncrementor {
    data: Vec<usize>,
    max_amount: usize,
}

impl NIncrementor {
    fn iter_mut(&mut self) -> &mut Self {
        self
    }

    fn new(amount: usize, max_amount: usize) -> Self {
        Self {
            data: vec![0; amount],
            max_amount,
        }
    }
}

impl Iterator for NIncrementor {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data[self.data.len()-1] >= self.max_amount {return None;}
        let result = self.data.clone();

        for i in 0..self.data.len() {
            if i < self.data.len()-1 && self.data[i] > self.max_amount {
                self.data[i] = 0;
            } else {
                self.data[i] += 1;
                break;
            }
        }

        return Some(result);
    }
}

#[derive(Clone)]
struct Vector(Vec<f64>);

impl Vector {
    fn is_negative(&self) -> bool {
        self.0.iter().any(|&val|{val <= -0.5})
    }

    fn is_whole(&self) -> bool {
        self.0.iter().all(|&val|{val.fract()<0.1 || val.fract()>0.9})
    }
}

impl std::ops::Mul<usize> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut result = self.0.clone();
        for i in 0..result.len() {
            result[i] *= rhs as f64;
        };
        return Vector(result);
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.0.clone();
        for i in 0..result.len() {
            result[i] -= rhs.0[i];
        };
        return Vector(result);
    }
}

// Huge thanks to https://youtu.be/i7f9PBe-j_Y for explaining it
// However I got errors with original version, so this one is somewhat modified
fn gauss_jordan(equations: &Vec<Vec<f64>>, results: &Vec<f64>) -> (Vec<Vec<f64>>, Vector, Vec<Vector>) {
    let mut free_variables: Vec<usize> = Vec::new();
    let n_columns = equations[0].len();
    let n_rows = equations.len();
    let mut solution = equations.clone();
    let mut solution_result = results.clone();

    for i_diagonal in 0.. {
        if i_diagonal >= n_columns {break;}
        let pivot_row = i_diagonal-free_variables.len();

        // hit bottom, so everything after is free
        if pivot_row >= n_rows {
            free_variables.push(i_diagonal);
            continue;
        }

        // Partial pivoting
        if solution[pivot_row][i_diagonal] == 0. {
            for y in pivot_row+1..n_rows {
                if solution[y][i_diagonal].abs() > solution[pivot_row][i_diagonal].abs() {
                    solution.swap(pivot_row, y);
                    (solution_result[pivot_row], solution_result[y]) = (solution_result[y], solution_result[pivot_row]);
                    break;
                }
            }
        }

        if solution[pivot_row][i_diagonal] == 0. {
            free_variables.push(i_diagonal);
            continue;
        }

        // Division of the pivot row
        let pivot = solution[pivot_row][i_diagonal]; // basically division factor
        for k in i_diagonal..n_columns {
            solution[pivot_row][k] /= pivot;
        }
        solution_result[pivot_row] /= pivot;

        // Elimination loop
        'down: for j in 0..n_rows {
            if j == pivot_row || solution[j][i_diagonal] == 0. {continue 'down;}
            let difference = solution[j][i_diagonal];
            for k in i_diagonal..n_columns {
                solution[j][k] -= difference * solution[pivot_row][k];
            }
            solution_result[j] -= difference * solution_result[pivot_row];
        }
    }
    let mut free_matricies: Vec<Vector> = Vec::new();
    for free_var in free_variables {
        free_matricies.push(
            Vector(solution.iter().map(|row|{
                row[free_var]
            }).collect())
        );
    }
    return (solution, Vector(solution_result), free_matricies);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
