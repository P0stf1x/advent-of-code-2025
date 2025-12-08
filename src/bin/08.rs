advent_of_code::solution!(8);

#[derive(Clone, Copy)]
#[derive(PartialEq)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    pub fn distance_squared(&self, other: &Self) -> f64 {
        (self.x-other.x).powi(2) +
        (self.y-other.y).powi(2) +
        (self.z-other.z).powi(2)
    }
}

struct JunctionStorage {
    junctions: Vec<Vec<Point3>>,
}

impl JunctionStorage {
    // Modifies self, and returns a clone of resulting structure
    pub fn connect(&mut self, point_a: Point3, point_b: Point3) -> Vec<Point3> {
        // We need to find which junction contain point_a and which contain point_b
        let index_a = self.junctions.iter().enumerate().filter(|(_, junction)|{junction.contains(&point_a)}).next().unwrap().0;
        let index_b = self.junctions.iter().enumerate().filter(|(_, junction)|{junction.contains(&point_b)}).next().unwrap().0;
        let smaller_index = index_a.min(index_b);
        let larger_index = index_a.max(index_b);

        if smaller_index == larger_index {return self.junctions[smaller_index].clone();}

        let mut other_junction = self.junctions.remove(larger_index);
        self.junctions[smaller_index].append(&mut other_junction);
        return self.junctions[smaller_index].clone();
    }

    pub fn calculate_largests(&self, amount: usize) -> Vec<u64> {
        let mut vector = self.junctions.iter()
            .map(|junction|{junction.len() as u64})
            .collect::<Vec<u64>>();
        vector.sort();
        vector[(vector.len()-amount)..].to_vec()
    }

    pub fn flat(&self) -> Vec<Point3> { // ugly. unpacks Vec<Vec<Point3>>
        self.junctions.iter().map(|a|{a[0]}).collect()
    }

    pub fn new(input: &str) -> Self {
        let junctions: Vec<Vec<Point3>> = input.lines().map(|line|{
            let mut split = line.split(',');
            vec![
                Point3 {
                    x: u64::from_str_radix(split.next().unwrap(), 10).unwrap() as f64,
                    y: u64::from_str_radix(split.next().unwrap(), 10).unwrap() as f64,
                    z: u64::from_str_radix(split.next().unwrap(), 10).unwrap() as f64,
                }
            ]
        }).collect();
        Self { junctions }
    }
}

fn vec2d_min<T>(vec2d: &Vec<Vec<T>>) -> (usize, usize)
where T: PartialOrd { // btw it's twice as slower than when comparing by value instead of reference. tldr 2x speed when T: Copy
    let total_points_x = vec2d[0].len();
    let total_points_y = vec2d.len();
    let mut min = &vec2d[0][0];
    let mut min_x = 0;
    let mut min_y = 0;
    for y in 0..total_points_y {
        for x in 0..total_points_x {
            if vec2d[x][y] < *min {
                min = &vec2d[x][y];
                min_x = x;
                min_y = y;
            }
        }
    };
    return (min_x, min_y);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut points = JunctionStorage::new(input);
    let points_indices = points.flat();

    // Calculating pairwise distances
    let total_points = points.junctions.len();
    // fill with f64::INFINITY instead of for example enum since it's easier to use, and there's no need to maintain afterwards
    let mut distance_matrix: Vec<Vec<f64>> = vec![vec![f64::INFINITY; total_points]; total_points];
    for y in 0..total_points {
        for x in 0..total_points {
            if x > y { // calculate only a to b, and not b to a. also exclude distance to self
                distance_matrix[x][y] = points.junctions[y][0].distance_squared(&points.junctions[x][0]);
            }
        }
    }

    #[cfg(test)]
    let connection_limit = 10;
    #[cfg(not(test))]
    let connection_limit = 1000;
    for _ in 0..connection_limit {
        let (min_x, min_y) = vec2d_min(&distance_matrix);
        distance_matrix[min_x][min_y] = f64::INFINITY;
        points.connect(points_indices[min_x], points_indices[min_y]);
    }

    let result = points.calculate_largests(3).iter().copied().reduce(|a, b|{a*b}).unwrap();
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points = JunctionStorage::new(input);
    let points_indices = points.flat();

    // Calculating pairwise distances
    let total_points = points.junctions.len();
    // fill with f64::INFINITY instead of for example enum since it's easier to use, and there's no need to maintain afterwards
    let mut distance_matrix: Vec<Vec<f64>> = vec![vec![f64::INFINITY; total_points]; total_points];
    for y in 0..total_points {
        for x in 0..total_points {
            if x > y { // calculate only a to b, and not b to a. also exclude distance to self
                distance_matrix[x][y] = points.junctions[y][0].distance_squared(&points.junctions[x][0]);
            }
        }
    }

    loop {
        let (min_x, min_y) = vec2d_min(&distance_matrix);

        // Basically remove pair from search
        distance_matrix[min_x][min_y] = f64::INFINITY;

        // Optimisation step
        // We also remove all permutations of nodes that are already in that junction
        // Technically it's unnecessary, since on average it would complete in a couple of thousands of iterations (a couple of seconds)
        // But it is theoretically possible for it to require almost 500k iterations which on my machine takes 15 minutes to compute
        let points_of_junction = points.connect(points_indices[min_x], points_indices[min_y]);
        let needs_removal: Vec<usize> = points_of_junction.iter().map(|point|{
            points_indices.iter().position(|p|{p==point}).unwrap()
        }).collect();
        for a in &needs_removal {
            for b in &needs_removal {
                distance_matrix[*a][*b] = f64::INFINITY;
            }
        }

        if points.junctions.len() == 1 {
            let result = (points_indices[min_y].x * points_indices[min_x].x) as u64;
            return Some(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
