use std::{collections::BinaryHeap, fs::read_to_string};

fn main() {
    let data = read_to_string("./data/12.txt").expect("Could not read data file");

    let hill = Hill::try_from(data.as_str()).expect("Could not parse hill");

    let shortest_path = find_shortest_path_from_start(&hill).expect("Could not find shortest path");
    println!("Part one: {}", shortest_path.0);

    let shortest_path = find_shortest_path_from_height(&hill, 0).expect("Could not find shortest path");
    println!("Part two: {}", shortest_path.0);
}

fn find_shortest_path_from_start(hill: &Hill) -> Option<(usize, Vec<usize>)> {
    let mut queue = BinaryHeap::new();

    queue.push(SearchThingy {
	cost: 0,
	data: hill.start_index,
    });

    find_shortest_path(hill, queue)

}

fn find_shortest_path_from_height(hill: &Hill, start_height: u32) -> Option<(usize, Vec<usize>)> {
    let mut queue = BinaryHeap::new();

    for start_index in hill.get_indices_at_height(start_height) {
	queue.push(SearchThingy {
	    cost: 0,
	    data: start_index,
	});
    }

    find_shortest_path(hill, queue)
}

fn find_shortest_path(
    hill: &Hill,
    mut queue: BinaryHeap<SearchThingy<usize>>,
) -> Option<(usize, Vec<usize>)> {
    let mut cache = vec![usize::MAX; hill.heights.len()];

    while let Some(next) = queue.pop() {
        for neigbour in hill.get_valid_neighbours(next.data) {
            let new_cost = next.cost + 1;
            if hill.end_index == neigbour {
                return Some((new_cost, cache));
            }

            if new_cost < cache[neigbour] {
                cache[neigbour] = new_cost;
                queue.push(SearchThingy {
                    cost: new_cost,
                    data: neigbour,
                });
            }
        }
    }

    None
}

struct Hill {
    heights: Vec<u32>,
    start_index: usize,
    end_index: usize,
    width: usize,
    height: usize,
}

impl Hill {
    fn get_neighbours(&self, index: usize) -> Vec<usize> {
        let mut neighbours = Vec::with_capacity(4);

        let row = index / self.width;
        let col = index % self.width;

        if row > 0 {
            neighbours.push(index - self.width);
        }
        if row < self.height - 1 {
            neighbours.push(index + self.width);
        }
        if col > 0 {
            neighbours.push(index - 1);
        }
        if col < self.width - 1 {
            neighbours.push(index + 1);
        }

        neighbours
    }

    fn get_valid_neighbours(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        let height = self.heights[index];
        self.get_neighbours(index)
            .into_iter()
            .filter(move |&neighbour| self.heights[neighbour] <= height + 1)
    }

    fn get_indices_at_height(&self, height: u32) -> impl Iterator<Item = usize> + '_ {
        self.heights
            .iter()
            .enumerate()
            .filter_map(move |(i, height2)| (*height2 == height).then_some(i))
    }
}

struct SearchThingy<T> {
    cost: usize,
    data: T,
}

impl<T> Ord for SearchThingy<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T> PartialOrd for SearchThingy<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for SearchThingy<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T> Eq for SearchThingy<T> {}

impl TryFrom<&str> for Hill {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let width = value
            .lines()
            .next()
            .ok_or("No lines when trying from &str")?
            .chars()
            .count();
        let height = value.lines().count();
        let mut start_index = None;
        let mut end_index = None;
        let heights = value
            .lines()
            .flat_map(|line| line.chars())
            .enumerate()
            .map(|(i, chr)| match chr {
                'S' => {
                    start_index = Some(i);
                    0
                }
                'E' => {
                    end_index = Some(i);
                    25
                }
                chr => chr as u32 - 'a' as u32,
            })
            .collect();

        let start_index = start_index.ok_or("Could not find start index")?;
        let end_index = end_index.ok_or("Could not find end index")?;

        Ok(Hill {
            heights,
            start_index,
            end_index,
            width,
            height,
        })
    }
}
