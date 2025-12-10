advent_of_code::solution!(9);

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Point2 {
    x: u64,
    y: u64,
}

impl Point2 {
    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

enum LineOrientation {
    Horizontal,
    Vertical,
}

struct Line {
    #[allow(dead_code)] // for debugging
    a: Point2,
    #[allow(dead_code)] // for debugging
    b: Point2,
    // To speedup it's easier to pre-compute
    orientation: LineOrientation,
    perpendicular_coord: u64, // x for vertical, y for horizontal
    smaller_coord: u64, // left/up edge
    bigger_coord: u64, // right/down edge
}

impl Line {
    fn intersects(&self, a: Point2, b: Point2) -> bool {
        let (a_parallel, b_parallel, a_perpendicular, b_perpendicular) = match self.orientation {
            LineOrientation::Vertical => {
                (a.y, b.y, a.x, b.x)
            }
            LineOrientation::Horizontal => {
                (a.x, b.x, a.y, b.y)
            }
        };
        if self.perpendicular_coord <= a_perpendicular.min(b_perpendicular) || self.perpendicular_coord >= a_perpendicular.max(b_perpendicular) {
            return false;
        } else {
            return self.smaller_coord < a_parallel.max(b_parallel) && self.bigger_coord > a_parallel.min(b_parallel);
        }
    }

    fn new(a: Point2, b: Point2) -> Self {
        let (orientation, perpendicular_coord) = match a.x==b.x {
            true => (LineOrientation::Vertical, a.x),
            false => (LineOrientation::Horizontal, a.y),
        };
        let smaller_coord = match orientation {
            LineOrientation::Vertical => a.y.min(b.y),
            LineOrientation::Horizontal => a.x.min(b.x)
        };
        let bigger_coord = match orientation {
            LineOrientation::Vertical => a.y.max(b.y),
            LineOrientation::Horizontal => a.x.max(b.x)
        };
        Self { a, b, orientation, perpendicular_coord, smaller_coord, bigger_coord }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let point_list: Vec<Point2> = input.lines().map(|line|{
        let (x_str, y_str) = line.split_once(',').unwrap();
        Point2::new(
            u64::from_str_radix(x_str, 10).unwrap(),
            u64::from_str_radix(y_str, 10).unwrap(),
        )
    }).collect();

    // The same as day 8. We pre calculate all pairs, and then start with largest until we find valid solution
    let total_points = point_list.len();
    let mut area_matrix: Vec<Vec<u64>> = vec![vec![0; total_points]; total_points];
    for y in 0..total_points {
        for x in 0..total_points {
            if x > y { // calculate only a to b, and not b to a. also exclude distance to self
                area_matrix[x][y] = point_list[y].area(&point_list[x]);
            }
        }
    }

    let result = area_matrix.iter().map(|l|{l.iter().max().unwrap()}).max().unwrap().clone();
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let point_list: Vec<Point2> = input.lines().map(|line|{
        let (x_str, y_str) = line.split_once(',').unwrap();
        Point2::new(
            u64::from_str_radix(x_str, 10).unwrap(),
            u64::from_str_radix(y_str, 10).unwrap(),
        )
    }).collect();

    // The same as day 8. We pre calculate all pairs, and then start with largest until we find valid solution
    let total_points = point_list.len();
    let mut area_matrix: Vec<Vec<u64>> = vec![vec![0; total_points]; total_points];
    for y in 0..total_points {
        for x in 0..total_points {
            if x > y { // calculate only a to b, and not b to a. also exclude area with self
                area_matrix[x][y] = point_list[y].area(&point_list[x]);
            }
        }
    }

    // I don't actually know a good algorithm for that and can't think of anything good, so we'll just check every line
    // Worst case is O(n^3) checks which is bad but not worst, and works fast enough with 500 inputs we are given
    let mut lines: Vec<Line> = Vec::with_capacity(total_points);
    for i in 0..total_points-1 {
        lines.push(Line::new(point_list[i], point_list[i+1]));
    }
    lines.push(Line::new(point_list[total_points-1], point_list[0]));

    // For easier iteration we store area matrix as tuple with both points and their area
    let mut points_pair_and_area: Vec<(Point2, Point2, u64)> = Vec::with_capacity((total_points.pow(2)-total_points)/2);
    for a_index in 0..total_points {
        for b_index in 0..total_points {
            if area_matrix[a_index][b_index] > 0 {
                points_pair_and_area.push((
                    point_list[a_index],
                    point_list[b_index],
                    area_matrix[a_index][b_index],
                ));
            }
        }
    }
    points_pair_and_area.sort_unstable_by(|tuple_a, tuple_b|{tuple_b.2.cmp(&tuple_a.2)});

    // We need to find biggest solution that does not intersect with any line
    'next_pair: for i in 0..points_pair_and_area.len() {
        for line in &lines {
            if line.intersects(
                points_pair_and_area[i].0,
                points_pair_and_area[i].1
            ) {continue 'next_pair;}
        }
        return Some(points_pair_and_area[i].2);
    }
    return None; // it's not possible to get there with valid input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
