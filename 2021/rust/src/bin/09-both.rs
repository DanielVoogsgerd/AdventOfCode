use std::{collections::{BinaryHeap, HashSet}, error::Error};

fn main() {
   let board = parse_file("./09-input.txt").unwrap();

   let low_points = board.get_low_points();
   let total_risk_score: u32 = low_points.iter().map(|index| {
      board.board[*index] as u32 + 1
   }).sum();
   println!("Total Risk score: {}", total_risk_score);

   let mut basin_sizes: BinaryHeap<usize> = board.find_basins().iter().map(|x| {x.len()}).collect::<BinaryHeap<usize>>();
   let product_largest_basins = (0..3).map(|_| {
      basin_sizes.pop().expect("Could not find enough basins")
   }).fold(1, |acc, size| {acc * size});

   println!("Product of largest basins {}", product_largest_basins);
}

fn parse_file(filename: &str) -> Result<Board, Box<dyn Error>> {
   let file = std::fs::read_to_string(filename)?;

   let width = file.lines().next().ok_or("Could not find first line")?.len();
   let height = file.lines().count();

   let data = file.lines().map(|x| {
      x.chars().map(|y| {
         y.to_digit(10).unwrap() as u8
      })
   }).flatten().collect::<Vec<u8>>();

   Ok(Board{
      board: data,
      width,
      height
   })
}

struct Board {
   board: Vec<u8>,
   width: usize,
   height: usize
}

impl Board {
   fn get_low_points(&self) -> Vec<usize> {
      (0..self.board.len()).filter(|index| {
         self.is_low_point(index)
      }).collect()
   }

   fn find_basins(&self) -> Vec<HashSet<usize>> {
      let mut basins: Vec<HashSet<usize>> = vec!();
      for low_index in self.get_low_points() {
         let mut basin: HashSet<usize> = HashSet::new();
         let mut queue: Vec<usize> = vec![low_index];

         while ! queue.is_empty() {
            let index = queue.pop().unwrap();

            self.get_neighbours(&index).iter().filter_map(|index| {
               if self.board[(*index)?] == 9 {
                  return None;
               } else {
                  return *index;
               }
            }).for_each(|neighbor_index| {
               if !basin.contains(&neighbor_index) {
                  basin.insert(neighbor_index);
                  queue.push(neighbor_index);
               }
            });
         }

         basins.push(basin);
      }

      return basins;
   }

   fn get_neighbours(&self, index: &usize) -> [Option<usize>; 4] {
      let (x, y) = self.get_coordinates(index);
      [
         self.get_index(x as i32 - 1, y as i32), self.get_index(x as i32 + 1, y as i32),
         self.get_index(x as i32, y as i32 - 1), self.get_index(x as i32, y as i32 + 1)
      ]
   }

   fn get_coordinates(&self, index: &usize) -> (usize, usize) {
      (index % self.width, index / self.width)
   }

   fn is_low_point(&self, index: &usize) -> bool {
      let local_value = self.board[*index];
      self.get_neighbours(index).iter().filter_map(|index: &Option<usize>| {
         Some(self.board[(*index)?] >= local_value)
      }).fold(true, |acc, is_lower| {acc && is_lower})
   }

   fn get_index(&self, x: i32, y: i32) -> Option<usize> {
      if y >= self.height as i32 || y < 0 || x >= self.width as i32 || x < 0 {
         None
      } else {
         Some(self.width * y as usize + x as usize)
      }
   }
}
