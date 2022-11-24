use std::error::Error;

fn main() {
    let mut data = parse_file("22-input.txt").unwrap();
    data.reverse();

    let mut x_coords = data.iter()
        .map(|x| {x.1.0})
        .map(|x| { vec![x.0, x.1+1].into_iter() })
        .flatten()
        .collect::<Vec<_>>();

    let mut y_coords = data.iter()
        .map(|x| {x.1.1})
        .map(|x| { vec![x.0, x.1+1].into_iter() })
        .flatten()
        .collect::<Vec<_>>();

    let mut z_coords = data.iter()
        .map(|x| {x.1.2})
        .map(|x| { vec![x.0, x.1+1].into_iter() })
        .flatten()
        .collect::<Vec<_>>();

    let total_cuboids = reboot_reactor(&data, &mut x_coords, &mut y_coords, &mut z_coords);
    println!("Part 2: Enabled cuboids: {}", total_cuboids);

    x_coords.append(&mut vec![-50isize, 51]);
    y_coords.append(&mut vec![-50isize, 51]);
    z_coords.append(&mut vec![-50isize, 51]);
    x_coords.retain(|coord|{*coord >= -50 && *coord <= 51});
    y_coords.retain(|coord|{*coord >= -50 && *coord <= 51});
    z_coords.retain(|coord|{*coord >= -50 && *coord <= 51});

    let total_cuboids = reboot_reactor(&data, &mut x_coords, &mut y_coords, &mut z_coords);
    println!("Part 1: Enabled cuboids: {}", total_cuboids);
}

fn reboot_reactor(commands: &Vec<(bool, (Range, Range, Range))>, x_coords: &mut Vec<isize>, y_coords: &mut Vec<isize>, z_coords: &mut Vec<isize>) -> usize {
    x_coords.sort();
    y_coords.sort();
    z_coords.sort();
    x_coords.dedup();
    y_coords.dedup();
    z_coords.dedup();

    let mut total_size = 0;

    for x in x_coords.windows(2).into_iter() {
        let commands = commands.iter().filter(|&entry| {
            let x_range = entry.1.0;
            x[0] >= x_range.0 && x[0] <= x_range.1
        }).collect::<Vec<_>>();
        for y in y_coords.windows(2).into_iter() {
            let commands = commands.iter().filter(|&entry| {
                let y_range = entry.1.1;
                y[0] >= y_range.0 && y[0] <= y_range.1
            }).collect::<Vec<_>>();
            for z in z_coords.windows(2).into_iter() {
                for (command, (x_range, y_range, z_range)) in commands.iter() {
                    if z[0] >= z_range.0 && z[0] <= z_range.1 {
                        if *command {
                            total_size += (x[1] - x[0]) * (y[1] - y[0]) * (z[1] - z[0]);
                        }
                        break;
                    }
                }
            }
        }
    }

    total_size as usize
}

type Range = (isize, isize);

fn parse_file(filename: &str) -> Result<Vec<(bool, (Range, Range, Range))>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;

    let output = file.lines().map(|line| {
        let mut parts = line.split(' ');

        let command = parts.next().expect("No command founnd") == "on";
        let coord_str = parts.next().expect("No coordinates found");
        let mut ranges = coord_str.split(',').map(|range_str| {
            range_str.split("=").nth(1).unwrap()
        }).map(|range_str|{
            let mut parts = range_str.split("..").map(|x| { x.parse::<isize>().expect("Could not parse range boundary as integer") });
            (parts.next().unwrap(), parts.next().unwrap())
        });
        (command, (ranges.next().unwrap(), ranges.next().unwrap(), ranges.next().unwrap()))
    }).collect::<Vec<_>>();

    Ok(output)
}
