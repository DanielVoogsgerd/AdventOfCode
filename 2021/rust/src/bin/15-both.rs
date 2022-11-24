use std::collections::BinaryHeap;
use std::error::Error;

// Right now this works, but is clearly not a right architecture. I think this could be better solved by using composition.


fn main() {
    // let board = Board::from_file("./15-first-example-input.txt").unwrap();
    // let end_coord = (board.width()-1, board.height()-1);
    // let cost_vec = board.create_cost_vec((0, 0), end_coord);
    // println!("End cost: {}", cost_vec[board.index(end_coord)]);

    let board = ComplicatedBoard::from_file("./15-input.txt", 5, 5).unwrap();
    let end_coord = (board.width()-1, board.height()-1);
    let cost_vec = board.create_cost_vec((0, 0), end_coord);
    println!("End cost: {}", cost_vec[board.index(end_coord)]);
}

pub struct Board {
    pub fields: Vec<u8>,
    pub width: usize,
    pub height: usize
}

pub struct ComplicatedBoard {
    pub fields: Vec<u8>,
    width: usize,
    height: usize,

    pub times_width: usize,
    pub times_height: usize
}

impl ComplicatedBoard {
    pub fn from_file(filename: &str, times_width: usize, times_height: usize) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::read_to_string(filename)?;

        let fields: Vec<u8> = file.lines().map(|line| {
            line.chars().map(|digit| {
                digit.to_digit(10).expect("Could not parse digit") as u8
            })
        }).flatten().collect();

        let height = file.lines().count();
        let width = fields.len() / height;

        Ok(Self {
            fields,
            width,
            height,
            times_width,
            times_height
        })
    }

    fn width(&self) -> usize {
        self.width * self.times_width
    }

    fn height(&self) -> usize {
        self.height * self.times_height
    }

    fn field(&self, coord: (usize, usize)) -> u32 {
        let width_add_score = (coord.0 / self.width) as u32;
        let height_add_score = (coord.1 / self.height) as u32;
        
        let mapped_index = (coord.1 % self.height) * self.width + (coord.0 % self.width);

        (self.fields[mapped_index] as u32 + width_add_score + height_add_score - 1) % 9 + 1
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let output_iter = (0..(self.height())).map(|y|{
            (0..(self.width())).map(|x|{
                self.field((x, y)).to_string()
            }).collect::<String>()
        }).collect::<Vec<String>>();

        Ok(output_iter.join("\n"))
    }
}

impl Board {
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::read_to_string(filename)?;

        let fields: Vec<u8> = file.lines().map(|line| {
            line.chars().map(|digit| {
                digit.to_digit(10).expect("Could not parse digit") as u8
            })
        }).flatten().collect();

        let height = file.lines().count();
        let width = fields.len() / height;

        Ok(Self {
            fields,
            width,
            height
        })
    }

    fn index(&self, coord: (usize, usize)) -> usize {
        coord.1 * self.width + coord.0
    }

    fn field(&self, coord: (usize, usize)) -> u32 {
        self.fields[self.index(coord)] as u32
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let output_iter = (0..self.height()).map(|y|{
            (0..self.width).map(|x|{
                self.field((x, y)).to_string()
            }).collect::<String>()
        }).collect::<Vec<String>>();

        Ok(output_iter.join("\n"))
    }
}

impl PathFindable for ComplicatedBoard {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn field_cost(&self, coord: (usize, usize)) -> u32 {
        self.field(coord)
    }
}

impl PathFindable for Board {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn field_cost(&self, coord: (usize, usize)) -> u32 {
        self.field(coord) as u32
    }
}

trait PathFindable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn field_cost(&self, coord: (usize, usize)) -> u32;

    fn index(&self, coord: (usize, usize)) -> usize {
        coord.1 * self.width() + coord.0
    }

    fn coordinate(&self, position: usize) -> (usize, usize) {
        (position % self.width(), position / self.width())
    }
    
    fn euclidian_distance(&self, start: (usize, usize), end: (usize, usize)) -> u32 {
        (abs_diff(start.0, end.0) + abs_diff(start.1, end.1)) as u32
    }

    fn create_cost_vec(&self, start: (usize, usize), end: (usize, usize)) -> Vec<u32> {
        let mut fringe: BinaryHeap<State> = BinaryHeap::new();
        let start_heuristic = self.euclidian_distance(start, end);

        fringe.push(State{
            position: start,
            fringe_value: start_heuristic
        });

        let mut costs  = vec![std::u32::MAX; self.width()*self.height()];

        while fringe.len() > 0 {
            let state = fringe.pop().unwrap();
            let coord = state.position;

            let new_cost = state.fringe_value - self.euclidian_distance(coord, end);

            if costs[self.index(state.position)] <= new_cost { continue }
            costs[self.index(state.position)] = new_cost;

            for neighbour_coord in self.find_neighbours(coord) {
                let neighbour_cost = new_cost + self.field_cost(neighbour_coord);
                let new_fringe_value = neighbour_cost + self.euclidian_distance(neighbour_coord, end);
                fringe.push(State { position: neighbour_coord, fringe_value: new_fringe_value })
            }
        }

        return costs;
    }

    fn find_neighbours(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let offset = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        offset.iter().filter_map(|(o_x, o_y)| {
            let n_x = o_x + coord.0 as i32;
            let n_y = o_y + coord.1 as i32;
            
            if n_x < 0 || n_x >= self.width() as i32 || n_y < 0 || n_y >= self.height() as i32 {
                None
            } else {
                Some((n_x as usize, n_y as usize))
            }
        }).collect()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    fringe_value: u32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.fringe_value.cmp(&self.fringe_value)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y { x - y } else { y - x }
}