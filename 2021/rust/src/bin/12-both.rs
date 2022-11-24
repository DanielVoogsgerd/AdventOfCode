use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

fn main() {
    let graph = Graph::from_file("./12-input.txt").unwrap();
    let paths = graph.find_paths("start", "end");

    println!("Path count: {}", paths.len());
}

pub struct Graph {
    edges: BTreeMap<String, BTreeSet<String>>
}

impl Graph {
    pub fn from_file(path: &str) -> Result<Graph, Box<dyn Error>> {
        let file = std::fs::read_to_string(path)?;
        let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        file.lines().map(|line| {
            let parts = line.split('-').take(2).collect::<Vec<&str>>();
            (parts[0], parts[1])

        }).for_each(|(left, right)| {
            for (ref origin, ref destination) in [(left, right), (right, left)].iter() {
                match edges.get_mut(origin.clone()) {
                    None => {
                        let mut new_set = BTreeSet::new();
                        new_set.insert(String::from(*destination));
                        edges.insert(String::from(*origin), new_set);
                    }
                    Some(set) => {
                        set.insert(String::from(*destination));
                    }
                };
            }
        });

        Ok(Graph {
            edges
        })
    }

    pub fn find_paths(&self, start: &str, end: &str) -> Vec<Vec<String>> {
            self.find_path_recursive(&Vec::new(), start, end)
    }

    pub fn find_paths_with_duplicate(&self, start: &str, end: &str) -> Vec<Vec<String>> {
        self.find_path_with_duplicate_recursive(&Vec::new(), start, start, end, false)
    }

    fn find_path_recursive(&self, old_path: &Vec<String>, current: &str, end: &str) -> Vec<Vec<String>> {
        if current == end {
            let mut new_path = old_path.clone();
            new_path.push(String::from(current));
            return vec![new_path];
        } else {
            let mut path_collection :Vec<Vec<String>> = Vec::new();
            match Self::get_cave_type(current).unwrap() {
                CaveType::SMALL => {
                    // TODO: &String can't be right
                    if ! old_path.contains(&String::from(current)) {
                        let mut new_path = old_path.clone();
                        new_path.push(String::from(current));
                        for next in self.edges.get(current).unwrap() {
                            path_collection.extend(self.find_path_recursive(&new_path, next, end));
                        }
                    }
                },
                CaveType::BIG => {
                    let mut new_path = old_path.clone();
                    new_path.push(String::from(current));
                    for next in self.edges.get(current).unwrap() {
                        path_collection.extend(self.find_path_recursive(&new_path, next, end));
                    }
                }
            };

            path_collection
        }
    }

    fn find_path_with_duplicate_recursive(&self, old_path: &Vec<String>, current: &str, start: &str, end: &str, has_visited_twice: bool) -> Vec<Vec<String>> {
        if current == end {
            let mut new_path = old_path.clone();
            new_path.push(String::from(current));
            return vec![new_path];
        } else {
            let mut path_collection :Vec<Vec<String>> = Vec::new();
            match Self::get_cave_type(current).unwrap() {
                CaveType::SMALL => {
                    // TODO: &String can't be right
                    if ! old_path.contains(&String::from(current)) {
                        let mut new_path = old_path.clone();
                        new_path.push(String::from(current));
                        for next in self.edges.get(current).unwrap() {
                            path_collection.extend(self.find_path_with_duplicate_recursive(&new_path, next, start, end, has_visited_twice));
                        }
                    } else if ! has_visited_twice && current != start {
                        let mut new_path = old_path.clone();
                        new_path.push(String::from(current));
                        for next in self.edges.get(current).unwrap() {
                            path_collection.extend(self.find_path_with_duplicate_recursive(&new_path, next, start, end, true));
                        }
                    }
                },
                CaveType::BIG => {
                    let mut new_path = old_path.clone();
                    new_path.push(String::from(current));
                    for next in self.edges.get(current).unwrap() {
                        path_collection.extend(self.find_path_with_duplicate_recursive(&new_path, next, start, end, has_visited_twice));
                    }
                }
            };

            path_collection
        }
    }

    fn get_cave_type(name: &str) -> Result<CaveType, &str> {
        if name.chars().all(char::is_uppercase) {
            Ok(CaveType::BIG)
        } else if name.chars().all(char::is_lowercase) {
            Ok(CaveType::SMALL)
        } else {
            Err("Cave name should be either upper- or lowercase")
        }
    }
}

enum CaveType {
    SMALL,
    BIG
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn test_part_one_first_input() {
        let graph = Graph::from_file("./12-first-example-input.txt").unwrap();
        let paths = graph.find_paths("start", "end");

        assert_eq!(10, paths.len());
    }

    #[test]
    fn test_part_one_second_input() {
        let graph = Graph::from_file("./12-second-example-input.txt").unwrap();
        let paths = graph.find_paths("start", "end");

        assert_eq!(19, paths.len());
    }

    #[test]
    fn test_part_one_third_input() {
        let graph = Graph::from_file("./12-third-example-input.txt").unwrap();
        let paths = graph.find_paths("start", "end");

        assert_eq!(226, paths.len());
    }

    #[test]
    fn test_part_one_puzzle_input() {
        let graph = Graph::from_file("./12-input.txt").unwrap();
        let paths = graph.find_paths("start", "end");

        assert_eq!(4573, paths.len());
    }

    #[test]
    fn test_part_two_first_input() {
        let graph = Graph::from_file("./12-first-example-input.txt").unwrap();
        let paths = graph.find_paths_with_duplicate("start", "end");

        assert_eq!(36, paths.len());
    }

    #[test]
    fn test_part_two_second_input() {
        let graph = Graph::from_file("./12-second-example-input.txt").unwrap();
        let paths = graph.find_paths_with_duplicate("start", "end");

        assert_eq!(103, paths.len());
    }

    #[test]
    fn test_part_two_third_input() {
        let graph = Graph::from_file("./12-third-example-input.txt").unwrap();
        let paths = graph.find_paths_with_duplicate("start", "end");

        assert_eq!(3509, paths.len());
    }

    #[test]
    fn test_part_two_puzzle_input() {
        let graph = Graph::from_file("./12-input.txt").unwrap();
        let paths = graph.find_paths_with_duplicate("start", "end");

        assert_eq!(117509, paths.len());
    }
}