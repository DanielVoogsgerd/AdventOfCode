fn main() {
    // Part 1
    let file = std::fs::read_to_string("./18-input.txt").unwrap();
    let mut lines = file.lines();
    let start_line = lines.next().unwrap();
    let mut tree = parse_line(start_line);
    
    for line in lines {
        let add_tree = parse_line(line);
        tree = tree.add_reduce(&add_tree);
    }

    println!("Answer part 1: {}", tree.magnitude());

    // Part 2
    let trees: Vec<Tree> = file.lines().map(parse_line).collect();
    let mut max_magnitude = 0;
    for x_tree in &trees {
        for y_tree in &trees {
            let mut combined_tree = x_tree.add_reduce(&y_tree);
            let magnitude = combined_tree.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    println!("Answer part 2: {}", max_magnitude);
}


fn parse_line(input: &str) -> Tree {
    let mut data = Vec::new();

    let mut depth = 0;
    for character in input.chars() {
        match character {
            '[' => { depth += 1; }
            ']' => { depth -= 1; }
            ',' => {}
            ' ' => {}
            _ => { data.push((depth, character.to_digit(10).unwrap())) }
        }
    }

    Tree { data }
}

#[derive(Debug)]
struct Tree {
    data: Vec<(usize, u32)>
}

impl Tree {
    /// Explode uses the index of its leftmost item in the exploding node.
    fn explode(&mut self, index: usize) {
        if index >= 1 {
            self.data[index-1].1 += self.data[index].1
        }
        if index < self.data.len()-2 {
            self.data[index+2].1 += self.data[index+1].1
        }
        self.data[index] = (self.data[index].0 - 1, 0);
        self.data.remove(index+1);
    }

    fn split(&mut self, index: usize) {
        let (depth, val) = self.data[index];
        let left = val / 2;
        let right = val / 2 + val % 2;

        self.data[index] = (depth+1, left);
        self.data.insert(index+1, (depth+1, right));
    }

    fn reduce(&mut self) {
        loop {
            let mut explode_index: Option<usize> = None;
            for (index, (depth, _value)) in self.data.iter().enumerate() {
                if *depth > 4 {
                    explode_index = Some(index);
                    break;
                }
            }

            if let Some(index) = explode_index {
                self.explode(index);
                continue
            }

            let mut split_index: Option<usize> = None;
            for (index, (_depth, value)) in self.data.iter().enumerate() {
                if *value > 9 {
                    split_index = Some(index);
                    break;
                }
            }

            if let Some(index) = split_index {
                self.split(index);
                continue
            }

            break
        } 
    }

    fn add(&self, other: &Self) -> Tree {
        let data = self.data.iter()
        .chain(other.data.iter())
        .map(|(depth, val)| { (depth+1, *val) })
        .collect();
        Tree { data } 
    }

    fn add_reduce(&self, other: &Self) -> Tree {
        let mut new_tree = self.add(other);
        new_tree.reduce();
        new_tree
    }

    fn magnitude(&mut self) -> u32 {
        let max_depth = self.data.iter().map(|(depth, _val)|{*depth}).max().unwrap();

        for depth in (1..=max_depth).rev() {
            loop {
                let opt_index = self.data.iter().position(|&(new_depth, _val)| { depth == new_depth });
                if let Some(index) = opt_index {
                    let right = self.data.remove(index+1);
                    self.data[index] = (right.0-1, self.data[index].1*3 + right.1*2)
                } else {
                    break;
                }
            }
        }

        self.data[0].1
    }
}



#[cfg(test)]
mod tests {
    use crate::{parse_line, Tree};

    fn assert_tree(left: Tree, right: Tree) {
        let iter = right.data.iter();
        let check_iter = left.data.iter();
        let zipped_iter = iter.zip(check_iter);
        for (val, check) in zipped_iter {
            assert_eq!(val, check);
        }
    }

    #[test]
    fn test_parse_input() {
        let tree = parse_line("[[1,2], 3]");

        let mut iter = tree.data.iter();
        assert_eq!(Some(&(2, 1)), iter.next());
        assert_eq!(Some(&(2, 2)), iter.next());
        assert_eq!(Some(&(1, 3)), iter.next());
    }

    #[test]
    fn test_split() {
        let mut tree = Tree { data: vec![(2, 11), (2, 2), (1, 3)] };
        tree.split(0);

        let mut iter = tree.data.iter();
        assert_eq!(Some(&(3, 5)), iter.next());
        assert_eq!(Some(&(3, 6)), iter.next());
        assert_eq!(Some(&(2, 2)), iter.next());
        assert_eq!(Some(&(1, 3)), iter.next());
    }

    #[test]
    fn test_explode_with_neighbours() {
        let mut tree = parse_line("[4, [[1,2], 3]]");
        tree.explode(1);

        let mut iter = tree.data.iter();
        assert_eq!(Some(&(1, 5)), iter.next());
        assert_eq!(Some(&(2, 0)), iter.next());
        assert_eq!(Some(&(2, 5)), iter.next());
    }

    #[test]
    fn test_explode_without_right_neighbour() {
        let mut tree = parse_line("[1,[2, 3]]");
        tree.explode(1);

        let mut iter = tree.data.iter();
        assert_eq!(Some(&(1, 3)), iter.next());
        assert_eq!(Some(&(1, 0)), iter.next());
    }
    #[test]
    fn test_explode_without_left_neighbour() {
        let mut tree = parse_line("[[1,2], 3]");
        tree.explode(0);

        let mut iter = tree.data.iter();
        assert_eq!(Some(&(1, 0)), iter.next());
        assert_eq!(Some(&(1, 5)), iter.next());
    }

    #[test]
    fn test_example_input1() {
        let mut tree = parse_line("[[[[[9,8],1],2],3],4]");
        tree.reduce();

        let mut iter = tree.data.iter();
        assert_eq!(Some(&(4, 0)), iter.next());
        assert_eq!(Some(&(4, 9)), iter.next());
        assert_eq!(Some(&(3, 2)), iter.next());
        assert_eq!(Some(&(2, 3)), iter.next());
        assert_eq!(Some(&(1, 4)), iter.next());
    }

    #[test]
    fn test_example_input2() {
        let mut tree = parse_line("[[[[[9,8],1],2],3],4]");
        let check_tree = parse_line("[[[[0,9],2],3],4]");

        tree.reduce();

        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_example_input3() {
        let mut tree = parse_line("[7,[6,[5,[4,[3,2]]]]]");
        let check_tree = parse_line("[7,[6,[5,[7,0]]]]");

        tree.reduce();

        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_example_input4() {
        let mut tree = parse_line("[[6,[5,[4,[3,2]]]],1]");
        let check_tree = parse_line("[[6,[5,[7,0]]],3]");

        tree.reduce();

        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_example_input5() {
        let mut tree = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let check_tree = parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        tree.reduce();

        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_involved_example_input1() {
        let mut tree = parse_line("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let check_tree = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        tree.reduce();
        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_involved_example_input2() {
        let mut tree = parse_line("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");
        let check_tree = parse_line("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

        tree.reduce();
        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_involved_example_input3() {
        let mut tree = parse_line("[[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]");
        let check_tree = parse_line("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");

        tree.reduce();
        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_involved_example_input4() {
        let mut tree = parse_line("[[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]],[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]]");
        let check_tree = parse_line("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");

        tree.reduce();
        assert_tree(check_tree, tree);
    }

    #[test]
    fn test_addition1() {
        let mut first_tree = parse_line("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");
        let mut second_tree = parse_line("[7,[5,[[3,8],[1,4]]]]");
        let check_tree = parse_line("[[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]");

        let new_tree = first_tree.add(&second_tree);
        assert_tree(check_tree, new_tree);
    }

    #[test]
    fn test_addition2() {
        let mut first_tree = parse_line("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]");
        let mut second_tree = parse_line("[[2,[2,2]],[8,[8,1]]]");
        let check_tree = parse_line("[[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]],[[2,[2,2]],[8,[8,1]]]]");

        let new_tree = first_tree.add(&second_tree);
        assert_tree(check_tree, new_tree);
    }

    #[test]
    fn test_addition3() {
        let mut first_tree = parse_line("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]");
        let mut second_tree = parse_line("[2,9]");
        let check_tree = parse_line("[[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]");

        let new_tree = first_tree.add(&second_tree);
        assert_tree(check_tree, new_tree);
    }

    #[test]
    fn test_add_reduce1() {
        let mut first_tree = parse_line("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]");
        let mut second_tree = parse_line("[1,[[[9,3],9],[[9,0],[0,7]]]]");
        let check_tree = parse_line("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]");

        let mut new_tree = first_tree.add(&second_tree);
        new_tree.reduce();
        assert_tree(check_tree, new_tree);
    }

    #[test]
    fn test_add_reduce2() {
        let mut first_tree = parse_line("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]");
        let mut second_tree = parse_line("[[[5,[7,4]],7],1]");
        let check_tree = parse_line("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]");

        let mut new_tree = first_tree.add(&second_tree);
        new_tree.reduce();
        assert_tree(check_tree, new_tree);
    }

    #[test]
    fn test_add_reduce3() {
        let mut first_tree = parse_line("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]");
        let mut second_tree = parse_line("[[[[4,2],2],6],[8,7]]");
        let check_tree = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

        let mut new_tree = first_tree.add(&second_tree);
        new_tree.reduce();
        assert_tree(check_tree, new_tree);
    }

    #[test]
    fn test_magnitude_1() {
        let mut tree = parse_line("[[1,2],[[3,4],5]]");
        let new_tree = tree.magnitude();
        assert_eq!(143, new_tree);
    }

    #[test]
    fn test_magnitude_2() {
        let mut tree = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(1384, tree.magnitude());
    }

    #[test]
    fn test_magnitude_3() {
        let mut tree = parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(445, tree.magnitude());
    }

    #[test]
    fn test_magnitude_4() {
        let mut tree = parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(791, tree.magnitude());
    }

    #[test]
    fn test_magnitude_5() {
        let mut tree = parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(1137, tree.magnitude());
    }

    #[test]
    fn test_magnitude_6() {
        let mut tree = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(3488, tree.magnitude());
    }
}