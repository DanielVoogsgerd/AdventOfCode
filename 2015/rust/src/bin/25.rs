fn main() {
    let mut cur = 20151125u128;
    for _i in 1..get_index(2981, 3075) {
	cur = cur * 252533 % 33554393;
    }

    println!("{cur}");
}

fn get_index(row: usize, column: usize) -> usize {
    let total = row + column - 2;

    total*(total+1)/2 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index1() {
	assert_eq!(get_index(1, 1), 1);
    }
    #[test]
    fn test_get_index2() {
	assert_eq!(get_index(1, 2), 3);
    }
    #[test]
    fn test_get_index3() {
	assert_eq!(get_index(2, 1), 2);
    }
    #[test]
    fn test_get_index4() {
	assert_eq!(get_index(3, 3), 13);
    }
    #[test]
    fn test_get_index5() {
	assert_eq!(get_index(3, 4), 19);
    }
}
