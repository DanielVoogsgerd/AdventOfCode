use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("./data/14.txt").expect("Could not load datafile");
    let data = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let mut segments = line.split_whitespace();
            let speed = segments.nth(3)?.parse::<usize>().ok()?;
            let run_time = segments.nth(2)?.parse::<usize>().ok()?;
            let rest_time = segments.nth(6)?.parse::<usize>().ok()?;
            Some((speed.to_owned(), run_time.to_owned(), rest_time.to_owned()))
        })
        .collect::<Vec<_>>();

    let finish_time = 2503;

    let max_distance = distances_after(&data, finish_time)
        .max()
        .expect("Could not find maximum distance");

    println!("Part one: {max_distance}");

    let mut reindeer = vec![0; data.len()];

    println!("Reindeer count: {}", data.len());

    for cur_time in 1..(finish_time) {
        let distances = distances_after(&data, cur_time).collect::<Vec<_>>();
        let max_distance = distances.iter().max().expect("Could not find max distance");
        let winners = distances
            .iter()
            .enumerate()
            .filter_map(|(i, distance)| (distance == max_distance).then_some(i));

        for winner in winners {
            reindeer[winner] += 1;
        }
    }

    println!("{:#?}", reindeer);
}

fn distances_after(
    data: &[(usize, usize, usize)],
    cur_time: usize,
) -> impl Iterator<Item = usize> + '_ {
    data.iter().map(move |(speed, run_time, rest_time)| {
        let total_time = run_time + rest_time;

        let time_in_interval = cur_time % total_time;

        let ran_intervals = cur_time / total_time;

        let distance_before_interval = ran_intervals * speed * run_time;
        let distance_in_interval = usize::min(time_in_interval, *run_time) * speed;

        distance_in_interval + distance_before_interval
    })
}
