advent_of_code::solution!(5);

struct RangeStorage {
    ranges: Vec<Range>,
}

impl RangeStorage {
    fn add(&mut self, start: u64, end: u64) {
        self.ranges.push(Range {
            start,
            end,
        });
    }

    fn is_inside(&self, value: u64) -> bool {
        for range in &self.ranges {
            if range.is_inside(value) {
                return true;
            }
        };
        return false;
    }

    fn optimise(&mut self) {
        'one_more_time: loop {
            for i in 0..self.ranges.len()-1 {
                for j in i+1..self.ranges.len() {
                    let i_range = self.ranges[i];
                    let j_range = self.ranges[j];
                    if (i_range.start <= j_range.end) && (j_range.start <= i_range.end) {
                        self.ranges.remove(j);
                        self.ranges.remove(i);
                        self.ranges.insert(i, Range {
                            start: i_range.start.min(j_range.start),
                            end: i_range.end.max(j_range.end)
                        });
                        continue 'one_more_time;
                        // We changed vec length, so not to index out of bounds the easiest solution is to just iterate again.
                        // This will not affect performance in the slightest since there are 190 ranges to begin with, so at most
                        // it will run 189 addition times which is like 100µs tops
                    }
                }
            }
            break;
        }
    }

    fn new() -> Self {Self { ranges: vec![] }}
}

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn is_inside(&self, value: u64) -> bool {
        return value >= self.start && value <= self.end;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut good_ranges = RangeStorage::new();
    for line in ranges.lines() {
        let (start, end) = line.split_once("-").unwrap();
        good_ranges.add(
            u64::from_str_radix(start, 10).unwrap(),
            u64::from_str_radix(end, 10).unwrap()
        );
    }

    let mut result = 0;
    for line in ids.lines() {
        let value = u64::from_str_radix(line, 10).unwrap();
        if good_ranges.is_inside(value) {
            result += 1;
        }
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut good_ranges = RangeStorage::new();
    for line in ranges.lines() {
        let (start, end) = line.split_once("-").unwrap();
        good_ranges.add(
            u64::from_str_radix(start, 10).unwrap(),
            u64::from_str_radix(end, 10).unwrap()
        );
    }

    good_ranges.optimise();
    return Some(
        good_ranges.ranges.iter()
            .map(|range|{range.end-range.start+1}) // calculate length of each range
            .sum()
    );
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
        assert_eq!(result, Some(14));
    }
}
