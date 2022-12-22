use std::fs::read_to_string;

const DECRYPTION_KEY: isize = 811589153;

fn main() {
    let data = read_to_string("./data/20.txt").expect("Could not read data file");
    let answer = part_one(&data);
    println!("{answer}");

    let answer = part_two(&data);
    println!("{answer}");
}

fn part_one(data: &str) -> isize {
    let data = data
        .lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .collect::<Vec<_>>();

    let mut indices = (0..data.len()).collect::<Vec<_>>();
    // swap_all(&mut data, &mut indices);
    for i in 0..data.len() {
        let index = indices.iter().position(|&x| x == i).unwrap();
        let shift = data[i];

        swappy::<1>(shift, &mut indices, index);
    }

    let start_index = indices
        .iter()
        .map(|&i| data[i])
        .position(|x| x == 0)
        .expect("Could not find first zero");

    [1000, 2000, 3000]
        .iter()
        .map(|&x| (start_index + x) % data.len())
        .map(|i| data[indices[i]])
        .sum::<isize>()
}

fn swappy<const DECRYPTION_KEY: isize>(shift: isize, indices: &mut Vec<usize>, index: usize) {
    let new_index = (((index as isize) + shift * DECRYPTION_KEY).rem_euclid(indices.len() as isize - 1)) as usize;

    match new_index.cmp(&index) {
        std::cmp::Ordering::Less => {
            indices[new_index..=index].rotate_right(1);
        }
        std::cmp::Ordering::Greater => {
            indices[index..=new_index].rotate_left(1);
        }
        _ => {}
    }
}

fn swap_all<const DECRYPTION_KEY: isize>(data: &[isize], indices: &mut Vec<usize>) {
    for i in 0..data.len() {
        let index = indices.iter().position(|&x| x == i).unwrap();
        let shift = data[i];

        swappy::<DECRYPTION_KEY>(shift, indices, index);
    }
}

fn part_two(data: &str) -> isize {
    let data = data
        .lines()
        .filter_map(|line| line.parse::<isize>().ok())
        // .map(|x| x * DECRYPTION_KEY)
        .collect::<Vec<_>>();

    // let sort_data = data.iter().map(|&x| x % (data.len() as isize - 1)).collect::<Vec<_>>();

    let mut indices = (0..data.len()).collect::<Vec<_>>();

    for _ in 0..10 {
        swap_all::<DECRYPTION_KEY>(&data, &mut indices);
    }

    let start_index = indices
        .iter()
        .map(|&i| data[i])
        .position(|x| x == 0)
        .expect("Could not find first zero");

    // println!("{start_index}");

    [1000, 2000, 3000]
        .iter()
        .cloned()
        .map(|x| (start_index + x) % data.len())
        .map(|x| indices[x])
        .map(|i| data[i])
        // .inspect(|x| println!("{x}"))
        .sum::<isize>() * DECRYPTION_KEY
}


#[cfg(test)]
mod tests {
    fn rotate_start_zero(list: &mut [isize]) {
	let offset = list.iter().position(|&x| x == 0).unwrap();
	list.rotate_left(offset);
    }
    use super::*;

    #[test]
    fn test_swap_all() {
        let data = [1, 2, -3, 3, -2, 0, 4];
        let mut indices = (0..data.len()).collect::<Vec<_>>();

        swap_all::<1>(&data, &mut indices);

        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, 3, -2, 1, 2, -3, 4]);
    }

    #[test]
    fn test_swap_all_twice() {
        let data = [1, 2, -3, 3, -2, 0, 4];
        let mut indices = (0..data.len()).collect::<Vec<_>>();

        for _ in 0..2 {
            swap_all::<1>(&data, &mut indices);
        }

        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, -3, 1, 4, 2, 3, -2]);
    }

    #[test]
    fn test_swap_neg_no_wrap() {
        let data = [0, 1, 2, 3, -2, 5];
        let mut indices = (0..data.len()).collect::<Vec<_>>();
        let shift = data[4];
        swappy::<1>(shift, &mut indices, 4);
        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, 1, -2, 2, 3, 5]);
    }

    #[test]
    fn test_swap_neg_to_last() {
        let data = [0, 1, -2, 3, 4, 5];
        let mut indices = (0..data.len()).collect::<Vec<_>>();
        let shift = data[2];
        swappy::<1>(shift, &mut indices, 2);
        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, 1, 3, 4, 5, -2]);
    }

    #[test]
    fn test_swap_pos_no_wrap() {
        let data = [0, 1, 2, 3, 4, 5];
        let mut indices = (0..data.len()).collect::<Vec<_>>();
        let shift = data[2];
        swappy::<1>(shift, &mut indices, 2);
        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, 1, 3, 4, 2, 5]);
    }

    #[test]
    fn test_swap_pos_wrap() {
        let data = [0, 1, 2, 3, 4, 5];
        let mut indices = (0..data.len()).collect::<Vec<_>>();
        let shift = data[4];
        swappy::<1>(shift, &mut indices, 4);
        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, 1, 2, 4, 3, 5]);
    }

    #[test]
    fn test_swap_pos_longer_than_list() {
        let data = [0, 1, 2, 13, 4, 5];
        let mut indices = (0..data.len()).collect::<Vec<_>>();
        let shift = data[3];
        swappy::<1>(shift, &mut indices, 3);
        let mut new_order = indices.iter().map(|&i| data[i]).collect::<Vec<_>>();
	rotate_start_zero(&mut new_order);
        assert_eq!(new_order, [0, 13, 1, 2, 4, 5]);
    }

    #[test]
    fn test_part_one() {
        let data =
            read_to_string("./data/20-example.txt").expect("Could not read example data file");
        assert_eq!(part_one(&data), 3);
    }

    #[test]
    fn test_part_two() {
        let data =
            read_to_string("./data/20-example.txt").expect("Could not read example data file");
        assert_eq!(part_two(&data), 1623178306);
    }

    #[test]
    fn test_part_two_first_iteration() {
        let data = [1, 2, -3, 3, -2, 0, 4];
        let mut indices = (0..data.len()).collect::<Vec<_>>();

        swap_all::<DECRYPTION_KEY>(&data, &mut indices);

        let mut new_order = indices
            .iter()
            .cloned()
            .map(|i| data[i])
            .collect::<Vec<_>>();

	rotate_start_zero(&mut new_order);

        assert_eq!(new_order, [0, -3, 4, -2, 3, 2, 1]);
    }

    #[test]
    fn test_part_two_second_iteration() {
	// 0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153
        let data = [1, 2, -3, 3, -2, 0, 4];
        let mut indices = (0..data.len()).collect::<Vec<_>>();

	for _i in 0..2 {
	    swap_all::<DECRYPTION_KEY>(&data, &mut indices);
	}

        let mut new_order = indices
            .iter()
            .cloned()
            .map(|i| data[i])
            .collect::<Vec<_>>();

	rotate_start_zero(&mut new_order);

        assert_eq!(new_order, [0, 3, 2, 4, -3, -2, 1]);
    }
}
