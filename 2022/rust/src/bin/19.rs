use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs::read_to_string,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use nom_supreme::ParserExt;

type Cost = (u32, Material);
type Blueprint = Vec<Robot>;

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    producing: Material,
    costs: Vec<Cost>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct State {
    time: u32,
    robots: BTreeMap<Material, u32>,
    ore: BTreeMap<Material, u32>,
}

fn main() {
    let data = read_to_string("./data/19.txt").expect("Could not load data file");
    println!("Part one: {}", part_one(&data));
    println!("Part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let blueprints = data
        .lines()
        .map(|line| parse_blueprint(line).ok().unwrap().1);

    blueprints
        .map(|blueprint| find_optimum::<24>(&blueprint))
        .enumerate()
        .map(|(i, best)| (i + 1) * best as usize)
        .sum()
}

fn part_two(data: &str) -> usize {
    let blueprints = data
        .lines()
        .map(|line| parse_blueprint(line).ok().unwrap().1);

    blueprints
        .take(3)
        .map(|blueprint| find_optimum::<32>(&blueprint))
        .inspect(|x| println!("{x}"))
        .product::<u32>() as usize
}

fn time_till_new_bot(
    robot: &Robot,
    current_materials: &BTreeMap<Material, u32>,
    producing: &BTreeMap<Material, u32>,
) -> u32 {
    robot
        .costs
        .iter()
        .map(|(cost, material)| {
            if let Some(flow) = producing.get(material) {
                let current = current_materials.get(&material).unwrap_or(&0);
                if current >= cost {
                    return 0;
                }
                let needed = cost - current;
                (needed / flow) + if needed % flow == 0 { 0 } else { 1 }
            } else {
                u32::MAX / 8
            }
        })
        .max()
        .unwrap()
}

fn find_optimum<const MAX_TIME: u32>(blueprint: &[Robot]) -> u32 {
    let mut max_per_material = BTreeMap::new();
    for robot in blueprint {
        for (amount, material) in &robot.costs {
            let current_max = max_per_material.entry(material).or_insert(0);
            *current_max = u32::max(*current_max, *amount);
        }
    }

    max_per_material.insert(&Material::Geode, u32::MAX);

    let mut queue: VecDeque<State> = Default::default();

    let mut initial_robots = BTreeMap::new();
    initial_robots.insert(Material::Ore, 1);

    queue.push_back(State {
        robots: initial_robots,
        time: 0,
        ore: Default::default(),
    });

    let mut best = 0;
    while let Some(current) = queue.pop_back() {
        let time_left = MAX_TIME - current.time;
        best = u32::max(
            best,
            current.ore.get(&Material::Geode).unwrap_or(&0)
                + time_left * current.robots.get(&Material::Geode).unwrap_or(&0),
        );

        for new_type in blueprint {
            if max_per_material.get(&new_type.producing).unwrap_or(&0)
                <= current.robots.get(&new_type.producing).unwrap_or(&0)
            {
                continue;
            }

            let time_needed = time_till_new_bot(new_type, &current.ore, &current.robots) + 1;
            if current.time + time_needed > MAX_TIME {
                continue;
            }

            let mut new = current.clone();
            new.time += time_needed;

            // Add amount of material the other robots will build until that is completed
            for (material, amount) in &new.robots {
                *new.ore.entry(material.clone()).or_insert(0) += amount * time_needed;
            }

            // Subtract amount needed to built robot
            for (cost_amount, cost_material) in &new_type.costs {
                *new.ore.get_mut(cost_material).unwrap() -= cost_amount;
            }

            *new.robots.entry(new_type.producing.clone()).or_insert(0) += 1;

            queue.push_back(new.clone());
        }
    }
    best
}

fn parse_blueprint(data: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(tag("."), parse_robot)(data)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = take_until("Each")(input)?;
    let (input, _) = take(5usize)(input)?;

    let (input, producing) = parse_material(input)?;

    let (input, _) = take_until("costs")(input)?;
    let (input, _) = take(6usize)(input)?;

    let (input, costs) = separated_list1(
        tag(" and "),
	parse_cost
    )(input)?;

    IResult::Ok((input, Robot { producing, costs }))
}

fn parse_cost(input: &str) -> IResult<&str, Cost> {
    separated_pair(complete::u32, tag(" "), parse_material)(input)
}

fn parse_material(input: &str) -> IResult<&str, Material> {
    alt((
        tag("ore").value(Material::Ore),
        tag("clay").value(Material::Clay),
        tag("obsidian").value(Material::Obsidian),
        tag("geode").value(Material::Geode),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

        assert_eq!(
            parse_blueprint(line).unwrap().1,
            vec![
                Robot {
                    producing: Material::Ore,
                    costs: vec![(4, Material::Ore)]
                },
                Robot {
                    producing: Material::Clay,
                    costs: vec![(2, Material::Ore)]
                },
                Robot {
                    producing: Material::Obsidian,
                    costs: vec![(3, Material::Ore), (14, Material::Clay)]
                },
                Robot {
                    producing: Material::Geode,
                    costs: vec![(2, Material::Ore), (7, Material::Obsidian)]
                },
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let data =
            read_to_string("./data/19-example.txt").expect("Could not load example data file");
        let answer = part_one(&data);
        assert_eq!(answer, 33);
    }

    // #[test]
    // fn test_part_two() {
	
    // }
}
