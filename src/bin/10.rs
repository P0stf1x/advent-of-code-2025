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
        println!("{diagram}={result:X}");
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
        println!("{bits_str}={result:X}");
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
    println!("{:?}", lines_split[0]);
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
    None
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
        assert_eq!(result, None);
    }
}
