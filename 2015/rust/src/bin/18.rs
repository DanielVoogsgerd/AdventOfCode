use std::fs::read_to_string;

fn main() {
    part_one();
    part_two();
}

const ON_STAY_ON_COUNT: [usize; 2] = [2, 3];
const OFF_TURN_ON_COUNT: [usize; 1] = [3];

fn part_one() {
    let data = read_to_string("./data/18.txt").expect("Could not read file");

    let stride = data.lines().next().expect("File is empty").chars().count();

    let mut data = data
        .lines()
        .flat_map(|line| line.chars().map(|chr| chr == '#'))
        .collect::<Vec<_>>();

    let height = data.len() / stride;

    for _iteration in 0..100 {
        data = iterate(&data, stride, height);
    }

    let count = data.into_iter().filter(|&state| state).count();

    println!("Part one: {count}");
}

fn part_two() {
    let data = read_to_string("./data/18.txt").expect("Could not read file");

    let stride = data.lines().next().expect("File is empty").chars().count();

    let mut data = data
        .lines()
        .flat_map(|line| line.chars().map(|chr| chr == '#'))
        .collect::<Vec<_>>();

    let height = data.len() / stride;

    // Turning on the corners
    data[get_index(0, 0, stride)] = true;
    data[get_index(stride - 1, 0, stride)] = true;
    data[get_index(0, height - 1, stride)] = true;
    data[get_index(stride - 1, height - 1, stride)] = true;

    for _iteration in 0..100 {
        data = iterate_with_stuck_corners(&data, stride, height);
    }

    let count = data.into_iter().filter(|&state| state).count();

    println!("Part two: {count}");
}

fn iterate(data: &[bool], stride: usize, height: usize) -> Vec<bool> {
    data.iter()
        .enumerate()
        .map(|(index, state)| {
            let (x, y) = get_coord(index, stride);

            let lit_neighbours = get_neighbours(x, y, stride, height)
                .map(|(x, y)| get_index(x, y, stride))
                .filter(|&neighbour_index| data[neighbour_index])
                .count();

            if *state {
                ON_STAY_ON_COUNT.contains(&lit_neighbours)
            } else {
                OFF_TURN_ON_COUNT.contains(&lit_neighbours)
            }
        })
        .collect::<Vec<_>>()
}

fn iterate_with_stuck_corners(data: &[bool], stride: usize, height: usize) -> Vec<bool> {
    let mut data = iterate(data, stride, height);

    data[get_index(0, 0, stride)] = true;
    data[get_index(stride - 1, 0, stride)] = true;
    data[get_index(0, height - 1, stride)] = true;
    data[get_index(stride - 1, height - 1, stride)] = true;

    data
}

fn get_neighbours(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    ((usize::max(x, 1) - 1)..=usize::min(width - 1, x + 1))
        .flat_map(move |x| {
            ((usize::max(y, 1) - 1)..=(usize::min(height - 1, y + 1))).map(move |y| (x, y))
        })
        .filter(move |(n_x, n_y)| !(*n_x == x && *n_y == y))
}

fn get_index(x: usize, y: usize, stride: usize) -> usize {
    y * stride + x
}

fn get_coord(index: usize, stride: usize) -> (usize, usize) {
    let x = index % stride;
    let y = index / stride;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbours() {
        let mut neighbours = get_neighbours(50, 50, 100, 100);

        assert_eq!(neighbours.next(), Some((49, 49)));
        assert_eq!(neighbours.next(), Some((49, 50)));
        assert_eq!(neighbours.next(), Some((49, 51)));
        assert_eq!(neighbours.next(), Some((50, 49)));
        assert_eq!(neighbours.next(), Some((50, 51)));
        assert_eq!(neighbours.next(), Some((51, 49)));
        assert_eq!(neighbours.next(), Some((51, 50)));
        assert_eq!(neighbours.next(), Some((51, 51)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_get_neighbours_top_left() {
        let mut neighbours = get_neighbours(0, 0, 100, 100);

        assert_eq!(neighbours.next(), Some((0, 1)));
        assert_eq!(neighbours.next(), Some((1, 0)));
        assert_eq!(neighbours.next(), Some((1, 1)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_get_neighbours_bottom_right() {
        let mut neighbours = get_neighbours(99, 99, 100, 100);

        assert_eq!(neighbours.next(), Some((98, 98)));
        assert_eq!(neighbours.next(), Some((98, 99)));
        assert_eq!(neighbours.next(), Some((99, 98)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_get_index() {
        assert_eq!(get_index(0, 0, 100), 0);
        assert_eq!(get_index(1, 0, 100), 1);
        assert_eq!(get_index(0, 1, 100), 100);
        assert_eq!(get_index(1, 1, 100), 101);
    }

    #[test]
    fn test_get_coord() {
        assert_eq!(get_coord(0, 100), (0, 0));
        assert_eq!(get_coord(1, 100), (1, 0));
        assert_eq!(get_coord(100, 100), (0, 1));
        assert_eq!(get_coord(101, 100), (1, 1));
    }
}
