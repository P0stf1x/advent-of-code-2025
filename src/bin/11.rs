use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::{Rc, Weak}};
// use std::

advent_of_code::solution!(11);

struct Node {
    name: String,
    connections: Vec<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            name,
            connections: Vec::new()
        }
    }

    fn find(&self, name: &str, include: &Vec<&str>) -> u64 {
        if self.name == name {
            match include.len() {
                0 => return 1,
                _ => return 0,
            };
        };
        let mut left_to_visit = include.clone();
        left_to_visit.retain(|to_visit|{*to_visit != self.name});
        let mut result = 0u64;
        for other in &self.connections {
            result += other.upgrade().unwrap().borrow().find(
                name,
                &left_to_visit,
            );
        };
        return result;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dictionary: HashMap<String, Rc<RefCell<Node>>> = HashMap::with_capacity(input.lines().count());
    for line in input.lines() {
        let name = line.split_once(':').unwrap().0;
        let node = match name {
            "you" => Node::new(String::from("you")),
            string => Node::new(String::from(string)),
        };
        dictionary.insert(name.to_string(), Rc::new(RefCell::new(node)));
    };
    dictionary.insert("out".to_string(), Rc::new(RefCell::new(Node::new("out".to_string()))));

    for line in input.lines() {
        let elements: Vec<&str> = line.split_whitespace().collect();
        let node = dictionary.get(&elements[0][0..3].to_string()).unwrap();
        for element in elements.iter().skip(1) {
            node.borrow_mut().connections.push(
                Rc::downgrade(dictionary.get(&element.to_string()).unwrap())
            );
        }
    }

    return Some(
        dictionary.get(&String::from("you")).unwrap().borrow().find("out", &vec![])
    );
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
