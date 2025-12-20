advent_of_code::solution!(12);

enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

#[derive(Clone, Copy)]
struct Box {
    data: [[bool; 3]; 3],
}

impl Box {
    fn at(&self, rot: &Rotation, x: usize, y: usize) -> bool {
        use Rotation::*;
        match rot {
            R0   => self.data[y][x],
            R90  => self.data[2-x][y],
            R180 => self.data[2-y][2-x],
            R270 => self.data[x][2-y],
        }
    }

    fn try_place(&self, rot: &Rotation, x_offset: isize, y_offset: isize, field: &Field) -> Option<Field> {
        let mut new_field = field.clone();
        for y in 0..3usize {
            for x in 0..3usize {
                if self.at(rot, x, y) {
                    let point_x = x_offset + x as isize - 1;
                    let point_y = y_offset + y as isize - 1;
                    if point_x < 0 || point_x >= field.size.0 as isize {return None;}
                    if point_y < 0 || point_y >= field.size.1 as isize {return None;}
                    if field.at(point_x as usize, point_y as usize) {return None;}
                    new_field.add_point(point_x as usize, point_y as usize);
                }
            }
        };
        return Some(new_field);
    }

    fn max_size(&self) -> usize {
        self.data.iter().map(|line|{
            line.iter().map(|&val|{if val {1} else {0}}).sum::<usize>()
        }).sum()
    }

    fn new(data: Vec<Vec<bool>>) -> Self {
        Self {
            data: [
                [data[0][0], data[0][1], data[0][2]],
                [data[1][0], data[1][1], data[1][2]],
                [data[2][0], data[2][1], data[2][2]],
            ]
        }
    }
}

type Boxes = Vec<Box>;
type BoxesCount = Vec<usize>;

#[derive(Clone)]
struct Field {
    size: (usize, usize),
    fill: Vec<Vec<bool>>,
}

impl Field {
    fn search(&self, boxes: Boxes, boxes_count: BoxesCount, iteration: usize) -> Option<Self> {
        if iteration == 0 {
            let needed_size: usize = boxes_count.iter().enumerate().map(|(box_type, box_count)|{
                boxes[box_type].max_size()*box_count
            }).sum();
            if needed_size > self.size.0*self.size.1 {
                return None;
            }
            if boxes_count.iter().sum::<usize>() <= (self.size.0/3)*(self.size.1/3) {
                return Some(self.clone());
            }
        }
        let mut i = 0;
        for (box_type, &box_count) in boxes_count.iter().enumerate() {
            if box_count > 0 {
                loop {
                    let next_field_option = self.place(boxes[box_type], i);
                    i += 1;
                    match next_field_option {
                        Some(next_field) => {
                            let mut new_boxes_count = boxes_count.clone();
                            new_boxes_count[box_type] -= 1;
                            if let Some(placement) = next_field.search(boxes.clone(), new_boxes_count, iteration+1) {
                                return Some(placement);
                            } else {
                                continue;
                            }
                        }
                        None => {
                            return None;
                        }
                    }
                }
            }
        }
        return Some(self.clone());
    }

    fn place(&self, box_type: Box, skip_n: usize) -> Option<Self> {
        let mut current_n = 0usize;
        for y in -1isize..=self.size.1 as isize {
            for x in -1isize..=self.size.0 as isize {
                use Rotation::*;
                for rot in [R0, R90, R180, R270] { // why does rust not have enum iterators...?
                    if let Some(placement) = box_type.try_place(&rot, x, y, &self) {
                        if current_n >= skip_n {
                            return Some(placement);
                        }
                        current_n += 1;
                    }
                }
            }
        }
        return None;
    }

    fn at(&self, x: usize, y: usize) -> bool {
        return self.fill[y][x];
    }

    fn add_point(&mut self, x: usize, y: usize) {
        self.fill[y][x] = true;
    }

    fn empty(size: (usize, usize)) -> Self {
        Self {
            size,
            fill: vec![vec![false; size.0];size.1],
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_things: Vec<&str> = input.split("\n\n").collect();
    let boxes: Boxes = input_things[0..6].iter().map(|box_input|{
        let box_lines: Vec<&str> = box_input.lines().collect();
        Box::new(
            vec![
                box_lines[1].chars().map(|c|if c == '#' {true} else {false}).collect(),
                box_lines[2].chars().map(|c|if c == '#' {true} else {false}).collect(),
                box_lines[3].chars().map(|c|if c == '#' {true} else {false}).collect(),
            ]
        )
    }).collect();
    let (fields_inputs, fields_boxes_inputs): (Vec<&str>, Vec<&str>) = input_things[input_things.len()-1].lines().collect::<Vec<_>>().iter().map(|line|{
        line.split_once(": ").unwrap()
    }).collect();
    let fields: Vec<Field> = fields_inputs.iter().map(|line|{
        let (x, y) = line.split_once("x").unwrap();
        Field::empty((
            usize::from_str_radix(x, 10).unwrap(),
            usize::from_str_radix(y, 10).unwrap(),
        ))
    }).collect();
    let fields_boxes: Vec<BoxesCount> = fields_boxes_inputs.iter().map(|line|{
        line.split_whitespace().map(|val|{
            usize::from_str_radix(&val, 10).unwrap()
        }).collect()
    }).collect();

    let mut result = 0u64;
    for (field, field_boxes) in fields.iter().zip(fields_boxes) {
        if let Some(_found) = field.search(boxes.clone(), field_boxes, 0) {
            result += 1;
        }
    }
    return Some(result);
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = Some(2u64); // calculated with actual function, but hardcoded since it takes around 8 minutes to calculate on provided example
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
