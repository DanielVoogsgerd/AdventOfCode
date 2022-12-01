use std::{collections::BinaryHeap, fmt::Display};

use itertools::{chain, iproduct};

fn main() {
    part_one();
    part_two();
}

#[derive(Clone, Debug, PartialEq)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn to_char(&self) -> char {
        match self {
            Pod::A => 'A',
            Pod::B => 'B',
            Pod::C => 'C',
            Pod::D => 'D',
        }
    }

    fn to_weight(&self) -> usize {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}

fn part_one() {
    let mut burrow: Burrow = Default::default();
    burrow.set_pod(Coordinate::Room(0, 0), Pod::D);
    burrow.set_pod(Coordinate::Room(0, 1), Pod::B);
    burrow.set_pod(Coordinate::Room(1, 0), Pod::A);
    burrow.set_pod(Coordinate::Room(1, 1), Pod::C);
    burrow.set_pod(Coordinate::Room(2, 0), Pod::D);
    burrow.set_pod(Coordinate::Room(2, 1), Pod::B);
    burrow.set_pod(Coordinate::Room(3, 0), Pod::C);
    burrow.set_pod(Coordinate::Room(3, 1), Pod::A);

    let mut solver = Solver::new(burrow);
    let cost = solver.solve().expect("Could not find solution").cost;
    println!("Part 1: {:?}", cost);
}

fn part_two() {
    let mut burrow = Burrow::new(4);
    burrow.set_pod(Coordinate::Room(0, 0), Pod::D);
    burrow.set_pod(Coordinate::Room(1, 0), Pod::A);
    burrow.set_pod(Coordinate::Room(2, 0), Pod::D);
    burrow.set_pod(Coordinate::Room(3, 0), Pod::C);

    burrow.set_pod(Coordinate::Room(0, 1), Pod::D);
    burrow.set_pod(Coordinate::Room(1, 1), Pod::C);
    burrow.set_pod(Coordinate::Room(2, 1), Pod::B);
    burrow.set_pod(Coordinate::Room(3, 1), Pod::A);

    burrow.set_pod(Coordinate::Room(0, 2), Pod::D);
    burrow.set_pod(Coordinate::Room(1, 2), Pod::B);
    burrow.set_pod(Coordinate::Room(2, 2), Pod::A);
    burrow.set_pod(Coordinate::Room(3, 2), Pod::C);

    burrow.set_pod(Coordinate::Room(0, 3), Pod::B);
    burrow.set_pod(Coordinate::Room(1, 3), Pod::C);
    burrow.set_pod(Coordinate::Room(2, 3), Pod::B);
    burrow.set_pod(Coordinate::Room(3, 3), Pod::A);

    let mut solver = Solver::new(burrow);
    let cost = solver.solve().expect("Could not find solution").cost;
    println!("Part 2: {:?}", cost);
}

#[derive(Clone, Debug, PartialEq)]
struct Burrow {
    hallway: [Option<Pod>; 7],
    rooms: [Vec<Option<Pod>>; 4],
}

type Index = usize;
type Depth = usize;
type Position = usize;
type Distance = usize;

type Move = (Coordinate, Coordinate);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Coordinate {
    Hallway(Index),
    Room(Index, Depth),
}

enum DisplayChar {
    Pod(Pod),
    Border,
    Empty,
}

impl DisplayChar {
    fn from_option(option: &Option<Pod>) -> Self {
        match option {
            Some(pod) => DisplayChar::Pod(pod.clone()),
            None => DisplayChar::Empty,
        }
    }

    fn to_char(&self) -> char {
        match self {
            DisplayChar::Pod(pod) => pod.to_char(),
            DisplayChar::Border => '#',
            DisplayChar::Empty => '.',
        }
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.rooms[0].len();
        let mut string = String::with_capacity(5 * 14);
        (0..13).for_each(|_| string.push('#'));
        string.push_str("\n#");
        (0..=10)
            .map(|position| {
                let coord = Self::get_coordinate(position);
                let display_char = match coord {
                    Coordinate::Hallway(index) => {
                        DisplayChar::from_option(&self.hallway[index as usize])
                    }
                    Coordinate::Room(_, _) => DisplayChar::Empty,
                };
                display_char.to_char()
            })
            .for_each(|character| string.push(character));
        for depth in 0..height {
            string.push_str("#\n#");
            (0..=10)
                .map(|position| {
                    let coord = Self::get_coordinate(position);
                    let display_char = match coord {
                        Coordinate::Hallway(_) => DisplayChar::Border,
                        Coordinate::Room(room_num, _) => {
                            DisplayChar::from_option(&self.rooms[room_num as usize][depth])
                        }
                    };
                    display_char.to_char()
                })
                .for_each(|character| string.push(character));
        }

        string.push_str("#\n");
        (0..=12).for_each(|_| string.push('#'));
        string.push('\n');

        write!(f, "{}", string)
    }
}

impl Default for Burrow {
    fn default() -> Self {
        Self::new(2)
    }
}

impl Burrow {
    fn new(height: usize) -> Self {
        let mut burrow = Self {
            hallway: Default::default(),
            rooms: Default::default(),
        };
        burrow
            .rooms
            .iter_mut()
            .for_each(|room| room.resize(height, None));
        burrow
    }

    fn set_pod(&mut self, coord: Coordinate, pod: Pod) {
        match coord {
            Coordinate::Hallway(hall_index) => self.hallway[hall_index] = Some(pod),
            Coordinate::Room(room_num, room_depth) => self.rooms[room_num][room_depth] = Some(pod),
        }
    }

    fn del_pod(&mut self, coord: Coordinate) {
        match coord {
            Coordinate::Hallway(hall_index) => self.hallway[hall_index] = None,
            Coordinate::Room(room_num, room_depth) => self.rooms[room_num][room_depth] = None,
        }
    }

    fn is_home(room_num: Index, pod: &Pod) -> bool {
        match pod {
            Pod::A => room_num == 0,
            Pod::B => room_num == 1,
            Pod::C => room_num == 2,
            Pod::D => room_num == 3,
        }
    }

    fn is_optimal(&self, room_num: Index, room_depth: Index, pod: &Pod) -> bool {
        let height = self.rooms[0].len();
        if !Self::is_home(room_num, pod) {
            return false;
        }

        ((room_depth + 1)..height).all(|depth| {
            if let Some(deeper_pod) = &self.rooms[room_num][depth] {
                deeper_pod == pod
            } else {
                false
            }
        })
    }

    fn get_home(pod: &Pod) -> usize {
        match pod {
            Pod::A => 0,
            Pod::B => 1,
            Pod::C => 2,
            Pod::D => 3,
        }
    }

    fn get_coordinate(position: Position) -> Coordinate {
        match position {
            0 | 1 => Coordinate::Hallway(position),
            3 => Coordinate::Hallway(position - 1),
            5 => Coordinate::Hallway(position - 2),
            7 => Coordinate::Hallway(position - 3),
            9 | 10 => Coordinate::Hallway(position - 4),
            2 | 4 | 6 | 8 => Coordinate::Room((position - 2) / 2, usize::MAX),
            _ => panic!("Invalid position"),
        }
    }

    fn is_complete(&self) -> bool {
        let hall_empty = self
            .hallway
            .iter()
            .all(|possible_pod| possible_pod.is_none());
        let pods_home = self.rooms.iter().enumerate().all(|(room_num, room)| {
            room.iter()
                .filter_map(|possible_pod| possible_pod.as_ref())
                .all(|pod| Self::is_home(room_num, pod))
        });
        hall_empty && pods_home
    }

    fn get_distance(from: Coordinate, to: Coordinate) -> Distance {
        match (from, to) {
            (Coordinate::Hallway(hall1_index), Coordinate::Hallway(hall2_index)) => isize::abs(
                Self::get_hallway_position(hall1_index) as isize
                    - Self::get_hallway_position(hall2_index) as isize,
            )
                as usize,
            (Coordinate::Hallway(hall_index), Coordinate::Room(room_num, room_depth))
            | (Coordinate::Room(room_num, room_depth), Coordinate::Hallway(hall_index)) => {
                isize::abs(
                    Self::get_hallway_position(hall_index) as isize
                        - Self::get_room_position(room_num) as isize,
                ) as usize
                    + room_depth
                    + 1
            }
            (
                Coordinate::Room(room1_num, room1_depth),
                Coordinate::Room(room2_num, room2_depth),
            ) => {
                if room1_num == room2_num {
                    isize::abs(room1_depth as isize - room2_depth as isize) as usize
                } else {
                    isize::abs(
                        Self::get_room_position(room1_num) as isize
                            - Self::get_room_position(room2_num) as isize,
                    ) as usize
                        + room1_depth
                        + room2_depth
                        + 2
                }
            }
        }
    }

    fn get_possible_moves(&self) -> impl Iterator<Item = Move> + '_ {
        // Maybe store them somewhere on Burrow
        let height = self.rooms[0].len();
        let room_coordinates = iproduct!((0..4), (0..height));
        let hallway_coordinates = 0..7;

        let (filled_room_coords, empty_room_coords): (Vec<(Index, Depth)>, Vec<(Index, Depth)>) =
            room_coordinates.partition(|coord| self.rooms[coord.0][coord.1].is_some());

        let filled_room_coords = filled_room_coords.into_iter().map(move |coord| {
            (
                coord,
                self.rooms[coord.0][coord.1]
                    .as_ref()
                    .expect("Could not find pod in filled room"),
            )
        });

        let (filled_hall_coords, empty_hall_coords): (Vec<Index>, Vec<Index>) =
            hallway_coordinates.partition(|&coord| self.hallway[coord].is_some());

        let filled_hall_coords = filled_hall_coords.into_iter().map(move |index| {
            (
                index,
                self.hallway[index]
                    .as_ref()
                    .expect("Could not find pod in filled hallway spot"),
            )
        });

        // Get all moves from pods in rooms to empty rooms
        let room_room_moves = iproduct!(filled_room_coords.clone(), empty_room_coords.clone())
            .filter(move |((start_room, pod), _)| !self.is_optimal(start_room.0, start_room.1, pod))
            .filter(move |((_, pod), end_room)| self.is_optimal(end_room.0, end_room.1, pod))
            .filter_map(move |((start_room, _), end_room)| {
                let start = Coordinate::Room(start_room.0, start_room.1);
                let end = Coordinate::Room(end_room.0, end_room.1);
                (!self.is_blocked(start, end)).then_some((start, end))
            });

        // Get all moves from pods in rooms to empry hallway slots
        let room_hall_moves = iproduct!(filled_room_coords, empty_hall_coords)
            .filter(move |((start_room, pod), _)| !self.is_optimal(start_room.0, start_room.1, pod))
            .filter_map(move |((start_room, _), end_index)| {
                let start = Coordinate::Room(start_room.0, start_room.1);
                let end = Coordinate::Hallway(end_index);
                (!self.is_blocked(start, end)).then_some((start, end))
            });

        // Get all moves from pods in hallways to empty rooms
        let hall_room_moves = iproduct!(filled_hall_coords, empty_room_coords)
            .filter(move |((_, pod), end_room)| self.is_optimal(end_room.0, end_room.1, pod))
            .filter_map(move |((start_index, _), end_room)| {
                let start = Coordinate::Hallway(start_index);
                let end = Coordinate::Room(end_room.0, end_room.1);
                (!self.is_blocked(start, end)).then_some((start, end))
            });

        chain!(room_room_moves, room_hall_moves, hall_room_moves)
    }

    fn make_move(
        mut self,
        start: Coordinate,
        end: Coordinate,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pod = self.get_pod(start).cloned().ok_or("Invalid move")?;
        self.del_pod(start);
        self.set_pod(end, pod);
        Ok(self)
    }

    fn get_hallway_position(index: Index) -> Position {
        match index {
            0 | 1 => index,
            2 => index + 1,
            3 => index + 2,
            4 => index + 3,
            5 | 6 => index + 4,
            _ => panic!("Invalid hallway index"),
        }
    }

    fn get_room_position(room_num: Index) -> Position {
        assert!(room_num < 4);
        room_num * 2 + 2
    }

    fn is_blocked(&self, from: Coordinate, to: Coordinate) -> bool {
        match (from, to) {
            (Coordinate::Hallway(hall1_index), Coordinate::Hallway(hall2_index)) => self.hallway
                [(hall1_index + 1)..=(hall2_index)]
                .iter()
                .any(|pod| pod.is_some()),
            (Coordinate::Room(room_num, room_depth), Coordinate::Hallway(hall_index))
            | (Coordinate::Hallway(hall_index), Coordinate::Room(room_num, room_depth)) => {
                let hall_position = Self::get_hallway_position(hall_index);
                let room_position = Self::get_room_position(room_num);

                let left = usize::min(hall_position, room_position);
                let right = usize::max(hall_position, room_position);

                let possible_hall_blocks = [(1, 1), (3, 2), (5, 3), (7, 4), (9, 5)];

                let hall_blocked = possible_hall_blocks.iter().any(|(position, index)| {
                    left < *position && *position < right && self.hallway[*index].is_some()
                });

                let room_blocked =
                    (0..room_depth).any(|depth| self.rooms[room_num][depth].is_some());

                // TODO: Early return or short-circuit
                hall_blocked || room_blocked
            }
            (
                Coordinate::Room(room1_num, room1_depth),
                Coordinate::Room(room2_num, room2_depth),
            ) => {
                let room1_position = Self::get_room_position(room1_num);
                let room2_position = Self::get_room_position(room2_num);

                let from = usize::min(room1_position, room2_position);
                let to = usize::max(room1_position, room2_position);

                let possible_hall_blocks = [(1, 1), (3, 2), (5, 3), (7, 4), (9, 5)];

                let hall_blocked = possible_hall_blocks.iter().any(|(position, index)| {
                    from < *position && *position < to && self.hallway[*index].is_some()
                });

                let room1_blocked =
                    (0..room1_depth).any(|depth| self.rooms[room1_num][depth].is_some());

                let room2_blocked =
                    (0..room2_depth).any(|depth| self.rooms[room2_num][depth].is_some());

                // TODO: Early return or short-circuit
                hall_blocked || room1_blocked || room2_blocked
            }
        }
    }

    fn get_pod(&self, start: Coordinate) -> Option<&Pod> {
        match start {
            Coordinate::Hallway(hall_index) => self.hallway[hall_index].as_ref(),
            Coordinate::Room(room_num, room_depth) => self.rooms[room_num][room_depth].as_ref(),
        }
    }
}

#[derive(Default)]
struct Solver {
    priority_queue: BinaryHeap<SolverState>,
}

impl Solver {
    fn new(begin_state: Burrow) -> Self {
        let mut solver = Self::default();
        solver.priority_queue.push(SolverState {
            heuristic: Self::compute_heuristic(&begin_state),
            burrow: begin_state,
            cost: 0,
        });
        solver
    }

    fn solve(&mut self) -> Option<SolverState> {
        while let Some(next_state) = self.priority_queue.pop() {
            let solved_state = self.step(next_state);
            if solved_state.is_some() {
                return solved_state;
            }
        }
        None
    }

    fn step(&mut self, next_state: SolverState) -> Option<SolverState> {
        let new_states = next_state
            .burrow
            .get_possible_moves()
            .filter_map(|(start, end)| {
                let pod = next_state
                    .burrow
                    .get_pod(start)
                    .expect("No pod found at start of move");
                let cost = Burrow::get_distance(start, end) * pod.to_weight();
                next_state
                    .burrow
                    .clone()
                    .make_move(start, end)
                    .ok()
                    .map(|burrow| (burrow, cost))
            })
            .map(|(new_burrow, cost)| {
                let new_cost = next_state.cost + cost;
                SolverState {
                    heuristic: Solver::compute_heuristic(&new_burrow) + new_cost,
                    burrow: new_burrow,
                    cost: new_cost,
                }
            });

        for new_state in new_states {
            if new_state.burrow.is_complete() {
                return Some(new_state);
            }
            self.priority_queue.push(new_state);
        }
        None
    }

    fn compute_heuristic(burrow: &Burrow) -> usize {
        let hallway_heuristic = burrow
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(index, possible_pod)| possible_pod.as_ref().map(|pod| (index, pod)))
            .map(|(index, pod)| {
                let home = Burrow::get_home(pod);
                Burrow::get_distance(Coordinate::Hallway(index), Coordinate::Room(home, 1))
                    * pod.to_weight()
            })
            .sum::<usize>();
        let room_heuristic = burrow
            .rooms
            .iter()
            .enumerate()
            .map(|(room_num, room)| {
                room.iter()
                    .enumerate()
                    .filter_map(|(room_depth, possible_pod)| {
                        possible_pod
                            .as_ref()
                            .map(|pod| ((room_num, room_depth), pod))
                    })
                    .map(|((room_num, room_depth), pod)| {
                        let home = Burrow::get_home(pod);
                        Burrow::get_distance(
                            Coordinate::Room(room_num, room_depth),
                            Coordinate::Room(home, 1),
                        ) * pod.to_weight()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        hallway_heuristic + room_heuristic
    }
}

#[derive(Debug)]
struct SolverState {
    heuristic: usize,
    burrow: Burrow,
    cost: usize,
}

impl Eq for SolverState {}

impl PartialEq for SolverState {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic.eq(&other.heuristic)
    }
}

impl Ord for SolverState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.partial_cmp(&other.heuristic).unwrap()
    }
}

impl PartialOrd for SolverState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heuristic.partial_cmp(&self.heuristic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance_hall_room() {
        let room = Coordinate::Room(0, 1);
        let hall = Coordinate::Hallway(3);
        assert_eq!(Burrow::get_distance(room, hall), 5);
        assert_eq!(Burrow::get_distance(hall, room), 5);
    }

    #[test]
    fn test_get_distance_hall_hall() {
        let hall1 = Coordinate::Hallway(3);
        let hall2 = Coordinate::Hallway(6);
        assert_eq!(Burrow::get_distance(hall1, hall2), 5);
        assert_eq!(Burrow::get_distance(hall2, hall1), 5);
    }

    #[test]
    fn test_get_distance_room_room() {
        let room1 = Coordinate::Room(0, 1);
        let room2 = Coordinate::Room(3, 0);
        assert_eq!(Burrow::get_distance(room1, room2), 9);
        assert_eq!(Burrow::get_distance(room2, room1), 9);

        let room1 = Coordinate::Room(2, 1);
        let room2 = Coordinate::Room(2, 1);
        assert_eq!(Burrow::get_distance(room1, room2), 0);
        assert_eq!(Burrow::get_distance(room2, room1), 0);
    }

    #[test]
    fn test_is_blocked_hall_hall() {
        let mut burrow = Burrow::default();
        let hall1 = Coordinate::Hallway(0);
        let hall2 = Coordinate::Hallway(6);

        assert!(!burrow.is_blocked(hall1, hall2));

        burrow.set_pod(Coordinate::Hallway(1), Pod::B);
        assert!(burrow.is_blocked(hall1, hall2));
    }

    #[test]
    fn test_is_blocked_hall_room() {
        let mut burrow = Burrow::default();
        let hall = Coordinate::Hallway(0);
        let room = Coordinate::Room(1, 1);

        assert!(!burrow.is_blocked(hall, room));
        assert!(!burrow.is_blocked(room, hall));

        burrow.set_pod(Coordinate::Hallway(1), Pod::B);
        assert!(burrow.is_blocked(hall, room));
        assert!(burrow.is_blocked(room, hall));

        let mut burrow: Burrow = Default::default();
        let hall = Coordinate::Hallway(2);
        let room = Coordinate::Room(1, 0);

        burrow.set_pod(hall, Pod::B);
        assert!(!burrow.is_blocked(hall, room));
        assert!(!burrow.is_blocked(room, hall));

        let room = Coordinate::Room(1, 1);

        assert!(!burrow.is_blocked(hall, room));
        assert!(!burrow.is_blocked(room, hall));
    }

    #[test]
    fn test_is_blocked_two_hallway() {
        let mut burrow = Burrow::default();
        let from = Coordinate::Hallway(2);
        let to = Coordinate::Room(0, 1);
        burrow.set_pod(Coordinate::Hallway(1), Pod::B);
        burrow.set_pod(from, Pod::A);

        assert!(!burrow.is_blocked(from, to));
    }

    #[test]
    fn test_is_blocked_room_room() {
        let mut burrow = Burrow::default();
        let room1 = Coordinate::Room(1, 0);
        let room2 = Coordinate::Room(2, 1);

        assert!(!burrow.is_blocked(room1, room2));
        assert!(!burrow.is_blocked(room2, room1));

        burrow.set_pod(Coordinate::Hallway(2), Pod::B);
        assert!(!burrow.is_blocked(room1, room2));
        assert!(!burrow.is_blocked(room2, room1));

        burrow.set_pod(Coordinate::Hallway(4), Pod::B);
        assert!(!burrow.is_blocked(room1, room2));
        assert!(!burrow.is_blocked(room2, room1));

        burrow.set_pod(Coordinate::Hallway(3), Pod::B);
        assert!(burrow.is_blocked(room1, room2));
        assert!(burrow.is_blocked(room2, room1));
    }

    #[test]
    fn test_optimal() {
        let burrow = Burrow::default();
        let pod = Pod::B;
        assert!(!burrow.is_optimal(1, 0, &pod));
        assert!(burrow.is_optimal(1, 1, &pod));
    }

    #[test]
    fn test_get_moves_from_hall() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Hallway(1), Pod::B);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&(Coordinate::Hallway(1), Coordinate::Room(1, 1))));

        burrow.set_pod(Coordinate::Hallway(2), Pod::C);
        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&(Coordinate::Hallway(2), Coordinate::Room(2, 1))));
    }

    #[test]
    fn test_get_moves_optimal() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(1, 1), Pod::B);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_get_moves_optimal_trapped() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(1, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(1, 1), Pod::A);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_get_moves_from_room() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(2, 1), Pod::A);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Hallway(0))));
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Hallway(1))));
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Hallway(2))));
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Hallway(3))));
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Hallway(4))));
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Hallway(5))));
        assert!(moves.contains(&(Coordinate::Room(2, 1), Coordinate::Room(0, 1))));

        burrow.set_pod(Coordinate::Room(2, 0), Pod::C);
        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 7);
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(0))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(1))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(2))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(3))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(4))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(5))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(6))));

        burrow.set_pod(Coordinate::Room(2, 0), Pod::B);
        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(0))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(1))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(2))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(3))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(4))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(5))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Hallway(6))));
        assert!(moves.contains(&(Coordinate::Room(2, 0), Coordinate::Room(1, 1))));
    }
    #[test]
    fn test_get_moves_trapped() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Hallway(2), Pod::A);
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_get_moves_switch_pods() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);
        burrow.set_pod(Coordinate::Hallway(2), Pod::A);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&(Coordinate::Room(0, 0), Coordinate::Hallway(0))));
        assert!(moves.contains(&(Coordinate::Room(0, 0), Coordinate::Hallway(1))));

        burrow = burrow
            .make_move(Coordinate::Room(0, 0), Coordinate::Hallway(1))
            .expect("Invalid move in test");
        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&(Coordinate::Hallway(2), Coordinate::Room(0, 1))));

        burrow = burrow
            .make_move(Coordinate::Hallway(2), Coordinate::Room(0, 1))
            .expect("Invalid move in test");
        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&(Coordinate::Hallway(1), Coordinate::Room(1, 1))));
    }

    #[test]
    fn test_is_complete() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(2, 1), Pod::A);
        assert!(!burrow.is_complete());

        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(1, 1), Pod::A);
        assert!(!burrow.is_complete());

        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(0, 1), Pod::A);
        assert!(burrow.is_complete());

        burrow.set_pod(Coordinate::Hallway(1), Pod::B);
        assert!(!burrow.is_complete());
    }

    #[test]
    fn test_heuristic() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Hallway(1), Pod::A);
        assert_eq!(Solver::compute_heuristic(&burrow), 4);

        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(2, 1), Pod::C);
        assert_eq!(Solver::compute_heuristic(&burrow), 0);
    }

    #[test]
    fn test_almost_solved() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Hallway(1), Pod::A);

        let mut solver = Solver::new(burrow);
        let solved_state = solver.solve();
        let cost = solved_state.expect("Could not solve burrow").cost;
        assert_eq!(cost, 3);
    }

    #[test]
    fn test_high_burrow() {
        let mut burrow = Burrow::new(4);
        burrow.set_pod(Coordinate::Hallway(2), Pod::A);
        burrow.set_pod(Coordinate::Hallway(4), Pod::C);

        let mut solver = Solver::new(burrow);
        let curr_state = solver
            .priority_queue
            .pop()
            .expect("Tried to pop from empty queue");
        let _ = solver.step(curr_state);

        let next_state = solver
            .priority_queue
            .pop()
            .expect("Tried to pop from empty queue");
        let cost = next_state.cost;
        println!("{}", next_state.burrow);
        assert_eq!(cost, 5);
    }

    #[test]
    fn test_high_burrow_case() {
        let mut burrow = Burrow::new(4);
        burrow.set_pod(Coordinate::Hallway(2), Pod::A);
        burrow.set_pod(Coordinate::Hallway(4), Pod::C);

        let moves = burrow.get_possible_moves().collect::<Vec<_>>();
        for found_move in &moves {
            println!("{:?}", found_move);
        }
        assert_eq!(moves.len(), 2)
    }

    #[test]
    fn test_trapped_pod() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Hallway(2), Pod::A);
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);

        let mut solver = Solver::new(burrow);
        let solved_state = solver.solve();
        let cost = solved_state.expect("Could not solve burrow").cost;
        assert_eq!(cost, 73);
    }

    #[test]
    fn test_trapped_pod_step() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Hallway(2), Pod::A);
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);
        let mut solver = Solver::new(burrow);

        let curr_state = solver
            .priority_queue
            .pop()
            .expect("Tried to pop from empty queue");
        assert_eq!(solver.step(curr_state), None);
        assert_eq!(solver.priority_queue.len(), 2);

        let curr_state = solver
            .priority_queue
            .pop()
            .expect("Tried to pop from empty queue");
        assert_eq!(curr_state.cost, 20);
        assert_eq!(curr_state.heuristic, 64 + 20);

        assert_eq!(solver.step(curr_state), None);
        assert_eq!(solver.priority_queue.len(), 2);

        let curr_state = solver
            .priority_queue
            .pop()
            .expect("Tried to pop from empty queue");
        assert_eq!(curr_state.cost, 23);
        assert_eq!(curr_state.heuristic, 60 + 23);

        let solved_state = solver.step(curr_state);
        assert!(solved_state.is_some());

        let solved_state = solved_state.expect("Solved state was not solved");
        assert_eq!(solved_state.cost, 73);
        assert_eq!(solved_state.heuristic, 0 + 73);
    }

    #[test]
    fn test_switch_pod() {
        let mut burrow = Burrow::default();
        burrow.set_pod(Coordinate::Room(1, 0), Pod::A);
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);

        let mut solver = Solver::new(burrow);
        let solved_state = solver.solve();
        let cost = solved_state.expect("Could not solve burrow").cost;
        assert_eq!(cost, 57);
    }

    #[test]
    fn test_example_step_by_step() {
        let mut burrow: Burrow = Default::default();
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(1, 0), Pod::C);
        burrow.set_pod(Coordinate::Room(2, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(3, 0), Pod::D);
        burrow.set_pod(Coordinate::Room(0, 1), Pod::A);
        burrow.set_pod(Coordinate::Room(1, 1), Pod::D);
        burrow.set_pod(Coordinate::Room(2, 1), Pod::C);
        burrow.set_pod(Coordinate::Room(3, 1), Pod::A);

        let fastests_moves = [
            (Coordinate::Room(2, 0), Coordinate::Hallway(2)),
            (Coordinate::Room(1, 0), Coordinate::Room(2, 0)),
            (Coordinate::Room(1, 1), Coordinate::Hallway(3)),
            (Coordinate::Hallway(2), Coordinate::Room(1, 1)),
            (Coordinate::Room(0, 0), Coordinate::Room(1, 0)),
            (Coordinate::Room(3, 0), Coordinate::Hallway(4)),
            (Coordinate::Room(3, 1), Coordinate::Hallway(5)),
            (Coordinate::Hallway(4), Coordinate::Room(3, 1)),
            (Coordinate::Hallway(3), Coordinate::Room(3, 0)),
            (Coordinate::Hallway(5), Coordinate::Room(0, 0)),
        ];
        let mut alter_burrow = burrow.clone();

        let mut solver = Solver::new(burrow);
        let mut next_state = solver
            .priority_queue
            .pop()
            .expect("Queue was empty after init");

        for next_move in fastests_moves {
            println!("{alter_burrow}");
            println!("{next_move:?}");
            alter_burrow = alter_burrow
                .make_move(next_move.0, next_move.1)
                .expect("Could not make a legal move");

            let solved = solver.step(next_state);
            if solved.is_some() {
                break;
            }
            next_state = solver
                .priority_queue
                .into_iter()
                .find(|state| state.burrow == alter_burrow)
                .expect("Expected move was not found");
            solver.priority_queue = BinaryHeap::new();
        }
    }

    #[test]
    fn test_example_step_by_step_get_moves() {
        let mut burrow: Burrow = Default::default();
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(1, 0), Pod::C);
        burrow.set_pod(Coordinate::Room(2, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(3, 0), Pod::D);
        burrow.set_pod(Coordinate::Room(0, 1), Pod::A);
        burrow.set_pod(Coordinate::Room(1, 1), Pod::D);
        burrow.set_pod(Coordinate::Room(2, 1), Pod::C);
        burrow.set_pod(Coordinate::Room(3, 1), Pod::A);

        burrow = burrow
            .make_move(Coordinate::Room(2, 0), Coordinate::Hallway(2))
            .expect("Failed to make a legal move");

        let new_move = (Coordinate::Room(1, 0), Coordinate::Room(2, 0));
        let possible_moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert!(possible_moves.contains(&new_move));
    }

    #[test]
    fn get_moves_room_room_shallow() {
        let mut burrow: Burrow = Default::default();
        burrow.set_pod(Coordinate::Room(1, 0), Pod::C);
        burrow.set_pod(Coordinate::Hallway(2), Pod::B);
        burrow.set_pod(Coordinate::Room(1, 1), Pod::D);
        burrow.set_pod(Coordinate::Room(2, 1), Pod::C);

        let possible_moves = burrow.get_possible_moves().collect::<Vec<_>>();
        assert_eq!(possible_moves.len(), 5);
        assert!(possible_moves.contains(&(Coordinate::Room(1, 0), Coordinate::Room(2, 0))));
        assert!(possible_moves.contains(&(Coordinate::Room(1, 0), Coordinate::Hallway(3))));
        assert!(possible_moves.contains(&(Coordinate::Room(1, 0), Coordinate::Hallway(4))));
        assert!(possible_moves.contains(&(Coordinate::Room(1, 0), Coordinate::Hallway(5))));
        assert!(possible_moves.contains(&(Coordinate::Room(1, 0), Coordinate::Hallway(6))));
    }

    #[test]
    fn test_example() {
        let mut burrow: Burrow = Default::default();
        burrow.set_pod(Coordinate::Room(0, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(1, 0), Pod::C);
        burrow.set_pod(Coordinate::Room(2, 0), Pod::B);
        burrow.set_pod(Coordinate::Room(3, 0), Pod::D);
        burrow.set_pod(Coordinate::Room(0, 1), Pod::A);
        burrow.set_pod(Coordinate::Room(1, 1), Pod::D);
        burrow.set_pod(Coordinate::Room(2, 1), Pod::C);
        burrow.set_pod(Coordinate::Room(3, 1), Pod::A);

        let mut solver = Solver::new(burrow);
        let solved_state = solver.solve();
        let cost = solved_state.expect("Could not solve burrow").cost;
        assert_eq!(cost, 12521);
    }
}
