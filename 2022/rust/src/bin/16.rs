use std::{collections::BTreeSet, fs::read_to_string};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until},
    character::complete,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

use itertools::Itertools;

fn main() {
    let data = read_to_string("./data/16.txt").expect("Could not read data file");
    let answer = part_one(&data);
    println!("{answer}");
    let answer = part_two_optimized(&data);
    println!("{answer}");
}

fn get_shortest_path(valves: &[(&str, usize, Vec<&str>)]) -> Vec<Vec<usize>> {
    let mut shortest_path = vec![vec![usize::MAX / valves.len(); valves.len()]; valves.len()];

    // Self distance is zero
    for i in 0..valves.len() {
        shortest_path[i][i] = 0;
    }

    // Fill shortest path with neigbours
    for (index, (_valve, _flow_rate, connected_valves)) in valves.iter().enumerate() {
        for connect_valve in connected_valves {
            // Linear search is probably faster than hashing
            let connected_index = valves
                .iter()
                .position(|(valve_name, _flow_rate, _connected_valves)| connect_valve == valve_name)
                .unwrap();

            shortest_path[index][connected_index] = 1;
        }
    }

    // Floyd's algorithm to find all pair shortests path
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                shortest_path[i][j] = usize::min(
                    shortest_path[i][j],
                    shortest_path[i][k] + shortest_path[k][j],
                );
            }
        }
    }

    shortest_path
}

fn part_one(data: &str) -> usize {
    let total_time = 30;

    let mut total_valves = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_, (valve, flow_rate, neighbours))| (valve, flow_rate as usize, neighbours))
        .collect::<Vec<_>>();

    total_valves.sort_by_key(|(_, flow_rate, _)| *flow_rate);
    total_valves.reverse();

    let shortest_path_all_valves = get_shortest_path(&total_valves);

    let flowing_valves = total_valves
        .iter()
        .enumerate()
        .filter_map(|(index, (_, flow_rate, _))| (*flow_rate > 0).then_some(index))
        .collect::<Vec<_>>();

    let flow_rates = total_valves
        .iter()
        .map(|(_, flow_rate, _)| *flow_rate)
        .collect::<Vec<_>>();

    // Since the starting position is not a flowing valve, we move to each flowing valve first
    let starting_index = total_valves
        .iter()
        .position(|(valves, _, _)| *valves == "AA")
        .expect("Could not find starting point");

    let starting_costs = flowing_valves
        .iter()
        .map(|i| shortest_path_all_valves[starting_index][*i]);

    let queue = get_start_queue(total_time, &flow_rates, &flowing_valves, starting_costs);

    run(
        total_time,
        &flow_rates,
        &flowing_valves,
        &shortest_path_all_valves,
        queue,
    )
    .1
}

fn get_start_queue(
    total_time: usize,
    flow_rates: &[usize],
    flowing_valves: &[usize],
    starting_costs: impl IntoIterator<Item = usize>,
) -> Vec<Vec<State>> {
    let mut queue = vec![vec![]; total_time];
    // For every starting point
    for (starting_valve_index, time_spent) in starting_costs.into_iter().enumerate() {
        let mut state = State {
            position: starting_valve_index,
            pressure: 0,
            flow_rate: flow_rates[flowing_valves[starting_valve_index]],
            valves: Default::default(),
        };
        state.valves.open_valve(starting_valve_index);
        queue[time_spent + 1].push(state);
    }

    queue
}

fn part_two_optimized(data: &str) -> usize {
    let total_time = 26;

    let mut total_valves = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_, (valve, flow_rate, neighbours))| (valve, flow_rate as usize, neighbours))
        .collect::<Vec<_>>();

    total_valves.sort_by_key(|(_, flow_rate, _)| *flow_rate);
    total_valves.reverse();

    let shortest_path_all_valves = get_shortest_path(&total_valves);

    let flowing_valves = total_valves
        .iter()
        .enumerate()
        .filter_map(|(index, (_, flow_rate, _))| (*flow_rate > 0).then_some(index))
        .collect::<Vec<_>>();

    let flow_rates = total_valves
        .iter()
        .map(|(_, flow_rate, _)| *flow_rate)
        .collect::<Vec<_>>();

    // Since the starting position is not a flowing valve, we move to each flowing valve first
    let starting_index = total_valves
        .iter()
        .position(|(valves, _, _)| *valves == "AA")
        .expect("Could not find starting point");

    let starting_costs = flowing_valves
        .iter()
        .map(|i| shortest_path_all_valves[starting_index][*i]);

    let queue = get_start_queue(total_time, &flow_rates, &flowing_valves, starting_costs);

    let solutions = run(
        total_time,
        &flow_rates,
        &flowing_valves,
        &shortest_path_all_valves,
        queue,
    )
    .0;

    let starting_costs = flowing_valves
        .iter()
        .map(|i| shortest_path_all_valves[starting_index][*i]);
    let queue = get_start_queue(30, &flow_rates, &flowing_valves, starting_costs);

    dbg!(run(
        30,
        &flow_rates,
        &flowing_valves,
        &shortest_path_all_valves,
        queue,
    )
    .0
    .into_iter()
    .max());

    let flowing_valves_set = flowing_valves.iter().cloned().collect::<BTreeSet<_>>();
    let partitioned_valves = (1..(flowing_valves.len() / 2))
        .flat_map(|len| flowing_valves.iter().cloned().combinations(len))
        .map(|chosen| {
            let set = chosen.iter().cloned().collect::<BTreeSet<_>>();
            (
                chosen,
                flowing_valves_set
                    .difference(&set)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
        // .inspect(|x| println!("{x:?}"))
        .map(|(h_valves, e_valves)| {
            let mut h_valves_obj = ValveCollection::default();
            h_valves_obj.open_valves(h_valves);

            let mut e_valves_obj = ValveCollection::default();
            e_valves_obj.open_valves(e_valves);

            solutions[h_valves_obj.open_valves] + solutions[e_valves_obj.open_valves]
        })
        // .collect::<Vec<_>>();
        .max()
        .expect("Could not find max");

    partitioned_valves
}

fn part_two(data: &str) -> usize {
    let mut total_valves = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_, (valve, flow_rate, neighbours))| (valve, flow_rate as usize, neighbours))
        .collect::<Vec<_>>();

    total_valves.sort_by_key(|(_, flow_rate, _)| *flow_rate);
    total_valves.reverse();

    let shortest_path_all_valves = get_shortest_path(&total_valves);

    let flowing_valves = total_valves
        .iter()
        .enumerate()
        .filter_map(|(index, (_, flow_rate, _))| (*flow_rate > 0).then_some(index))
        .collect::<Vec<_>>();

    let all_flow_rates = total_valves
        .iter()
        .map(|(_, flow_rate, _)| *flow_rate)
        .collect::<Vec<_>>();

    let flowing_valves_set = flowing_valves.iter().cloned().collect::<BTreeSet<_>>();

    let starting_index = total_valves
        .iter()
        .position(|(valves, _, _)| *valves == "AA")
        .expect("Could not find starting point");

    let run_time = 26;

    let partitioned_valves = (1..=(flowing_valves.len() / 2))
        .flat_map(|len| flowing_valves.iter().cloned().combinations(len))
        .map(|chosen| {
            let set = chosen.iter().cloned().collect::<BTreeSet<_>>();
            (
                chosen,
                flowing_valves_set
                    .difference(&set)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        });

    let mut i = 0;
    let answer = partitioned_valves
        .map(|(human_valves, elephant_valves)| {
            // dbg!(human_valves.clone());
            // dbg!(elephant_valves.clone());
            let human_starting_costs = human_valves
                .iter()
                .map(|&i| shortest_path_all_valves[starting_index][i]);

            let human_queue = get_start_queue(
                run_time,
                &all_flow_rates,
                &human_valves,
                human_starting_costs,
            );
            let human_pressure = run(
                run_time,
                &all_flow_rates,
                &human_valves,
                &shortest_path_all_valves,
                human_queue,
            )
            .1;

            let elephant_starting_costs = elephant_valves
                .iter()
                .map(|&i| shortest_path_all_valves[starting_index][i]);

            let elephant_queue = get_start_queue(
                run_time,
                &all_flow_rates,
                &elephant_valves,
                elephant_starting_costs,
            );
            let elephant_pressure = run(
                run_time,
                &all_flow_rates,
                &elephant_valves,
                &shortest_path_all_valves,
                elephant_queue,
            )
            .1;

            human_pressure + elephant_pressure
        })
        .inspect(|_| i += 1)
        .max()
        .expect("Could not find maximum pressure");

    dbg!(i);

    answer
}

fn run(
    run_time: usize,
    flow_rates: &[usize],
    chosen_valves: &[usize],
    shortest_path: &[Vec<usize>],
    mut queue: Vec<Vec<State>>,
) -> (Vec<usize>, usize) {
    let mut memo = vec![vec![vec![0; 1 << chosen_valves.len()]; chosen_valves.len()]; run_time];
    let mut optimal_pressure = 0;

    // dbg!(&chosen_valves);
    let mut solutions = vec![0; 1 << chosen_valves.len()];

    for time in 0..run_time {
        // dbg!(time);
        let time_left = run_time - time;
        let mut current_states = vec![];
        std::mem::swap(&mut current_states, &mut queue[time]);

        for state in current_states.drain(..) {
            let new_pressure = state.pressure + state.flow_rate * time_left;
            solutions[state.valves.open_valves] =
                usize::max(solutions[state.valves.open_valves], new_pressure);

            if state.valves.all_valves_open(chosen_valves.len()) {
                let new_pressure = state.pressure + state.flow_rate * time_left;
                optimal_pressure = usize::max(optimal_pressure, new_pressure);
                solutions[state.valves.open_valves] =
                    usize::max(solutions[state.valves.open_valves], new_pressure);
                continue;
            }

            if state.valves.has_open_valve(state.position) {
                for neighbour in state.valves.get_closed_valves(chosen_valves.len()) {
                    let time_spent =
                        shortest_path[chosen_valves[state.position]][chosen_valves[neighbour]];
                    if time_spent >= time_left {
                        let new_pressure = state.pressure + state.flow_rate * time_left;
                        optimal_pressure = usize::max(optimal_pressure, new_pressure);

                        solutions[state.valves.open_valves] =
                            usize::max(solutions[state.valves.open_valves], new_pressure);
                    } else {
                        let mut new_state = state.clone();
                        new_state.position = neighbour;
                        new_state.pressure += state.flow_rate * time_spent;

                        let new_time = time + time_spent;
                        if memo[new_time][new_state.position][new_state.valves.open_valves]
                            < new_state.pressure
                        {
                            memo[new_time][new_state.position][new_state.valves.open_valves] =
                                new_state.pressure;
                            queue[new_time].push(new_state);
                        }
                    }
                }
            } else {
                if 1 >= time_left {
                    let new_pressure = state.pressure + state.flow_rate;
                    optimal_pressure = usize::max(optimal_pressure, new_pressure);

                    solutions[state.valves.open_valves] =
                        usize::max(solutions[state.valves.open_valves], new_pressure);
                } else {
                    let mut new_state = state.clone();
                    new_state.valves.open_valve(state.position);
                    new_state.flow_rate += flow_rates[chosen_valves[state.position]];
                    new_state.pressure += state.flow_rate;

                    let end_pressure = new_state.pressure + (time_left - 1) * new_state.flow_rate;
                    solutions[new_state.valves.open_valves] =
                        usize::max(solutions[new_state.valves.open_valves], end_pressure);

                    let new_time = time + 1;

                    if memo[new_time][new_state.position][new_state.valves.open_valves]
                        < new_state.pressure
                    {
                        memo[new_time][new_state.position][new_state.valves.open_valves] =
                            new_state.pressure;
                        queue[new_time].push(new_state);
                    }
                }
            }
        }
    }

    dbg!(solutions
        .iter()
	.cloned()
	.enumerate()
        .filter_map(|(i, x)| (x == 0).then_some(i))
        .map(|valves| ValveCollection {
            open_valves: valves
        }
        .get_open_valves(64)
        .collect::<Vec<_>>())
        .collect::<Vec<_>>());

    (solutions, optimal_pressure)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MemoKey {
    position: BTreeSet<Position>,
    valves: ValveCollection,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    position: usize,
    time_till_arrival: usize,
}

#[derive(Clone, Debug)]
struct State {
    position: usize,
    pressure: usize,
    flow_rate: usize,
    valves: ValveCollection,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ValveCollection {
    open_valves: usize,
}

impl Default for ValveCollection {
    fn default() -> Self {
        Self { open_valves: 0 }
    }
}

impl ValveCollection {
    fn get_open_valves(&self, valve_count: usize) -> impl Iterator<Item = usize> + '_ {
        (0..valve_count).filter(|i| self.has_open_valve(*i))
    }

    fn get_closed_valves(&self, valve_count: usize) -> impl Iterator<Item = usize> + '_ {
        (0..valve_count).filter(|i| !self.has_open_valve(*i))
    }

    fn open_valve(&mut self, valve: usize) {
        self.open_valves |= 1 << valve;
    }

    fn open_valves(&mut self, valves: impl IntoIterator<Item = usize>) {
        for valve in valves {
            self.open_valve(valve);
        }
    }

    fn has_open_valve(&self, valve: usize) -> bool {
        (self.open_valves >> valve) & 1 == 1
    }

    fn all_valves_open(&self, valve_count: usize) -> bool {
        self.open_valves == (1 << valve_count) - 1
    }
}

fn parse_line(line: &str) -> IResult<&str, (&str, u64, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), take(2usize)),
        preceded(take_till(|c: char| c.is_ascii_digit()), complete::u64),
        preceded(
            alt((
                pair(take_until("valve "), tag("valve ")),
                pair(take_until("valves "), tag("valves ")),
            )),
            separated_list1(tag(", "), take(2usize)),
        ),
    ))(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data =
            read_to_string("./data/16-example.txt").expect("Could not read example data file");

        assert_eq!(part_one(&data), 1651);
    }

    #[test]
    fn test_part_two() {
        let data =
            read_to_string("./data/16-example.txt").expect("Could not read example data file");

        assert_eq!(part_two_optimized(&data), 1707);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Valve HH has flow rate=22; tunnel leads to valve GG")
                .unwrap()
                .1,
            ("HH", 22, vec!["GG"])
        );
        assert_eq!(
            parse_line("Valve GG has flow rate=0; tunnels lead to valves FF, HH")
                .unwrap()
                .1,
            ("GG", 0, vec!["FF", "HH"])
        );
    }

    #[test]
    fn test_open_valves() {
        let mut valves = ValveCollection::default();

        assert_eq!(valves.get_open_valves(10).count(), 0);

        valves.open_valve(4);

        let mut it = valves.get_open_valves(10);
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);
        drop(it);

        valves.open_valve(2);

        let mut it = valves.get_open_valves(10);
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);
        drop(it);
    }

    #[test]
    fn test_all_valves_open() {
        let mut valves = ValveCollection::default();
        valves.open_valve(0);
        valves.open_valve(1);
        valves.open_valve(2);

        assert!(!valves.all_valves_open(4));

        valves.open_valve(3);
        assert!(valves.all_valves_open(4));
    }
}
