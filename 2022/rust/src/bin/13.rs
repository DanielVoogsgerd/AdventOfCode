use std::fs::read_to_string;

use nom::{
    branch::alt, character::complete, combinator::map, multi::separated_list0, sequence::delimited,
    IResult,
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("./data/13.txt").expect("Could not load data file");

    let mut pairs = data.split("\n\n");

    let mut counter = 0;

    for (index, pair) in pairs.enumerate() {
        let mut packets = pair.lines();
        let first = packets.next().expect("Could not find first line in group");
        let second = packets.next().expect("Could not find second line in group");

        let packet1 = parse_list(first)
            .expect("Could not parse the first packet")
            .1;
        let packet2 = parse_list(second)
            .expect("Could not parse the second packet")
            .1;

        if packet1 < packet2 {
            counter += index + 1;
        }
    }

    println!("Part one: {counter}");
}

fn part_two() {
    let data = read_to_string("./data/13-example.txt").expect("Could not load data file");

    let mut items = data
        .lines()
        .filter_map(|line| parse_list(line).ok().map(|x| x.1))
        .collect::<Vec<_>>();
    let divider1 = parse_list("[[2]]").unwrap().1;
    let divider2 = parse_list("[[6]]").unwrap().1;

    items.push(divider1.clone());
    items.push(divider2.clone());
    items.sort();
    dbg!(&items);

    let mut item_iter = items.iter();

    let first_index = item_iter.by_ref().position(|x| *x == divider1).unwrap() + 1;
    let second_index = item_iter.by_ref().position(|x| *x == divider2).unwrap() + first_index + 1;

    let decoder_key = first_index * second_index;
    println!("Part two: {decoder_key}");
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Number(s), Item::Number(o)) => s.partial_cmp(o),
            (Item::Number(s), Item::List(o)) => Item::List(vec![self.clone()]).partial_cmp(other),
            (Item::List(_), Item::Number(_)) => self.partial_cmp(&Item::List(vec![other.clone()])),
            (Item::List(s), Item::List(o)) => s.partial_cmp(o),
        }
    }
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((parse_list, parse_number))(input)
}

fn parse_number(input: &str) -> IResult<&str, Item> {
    map(complete::u32, Item::Number)(input)
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    map(
        delimited(
            complete::char('['),
            separated_list0(complete::char(','), parse_item),
            complete::char(']'),
        ),
        Item::List,
    )(input)
}
