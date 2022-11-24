use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("./data/15.txt").expect("Could not load datafile");
    let data = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let mut segments = line.split(": ");
            let property_line = segments.nth(1)?;
            Some(property_line
                .split(", ")
                .filter_map(|property_segment| {
                    let mut x = property_segment.split(' ');
                    x.nth(1)?.parse::<isize>().ok()
                })
                .collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let total_teaspoons = 100;
    let total_ingredients = data.len();
    let total_properties = data[0].len();

    let recipes = total_sum(total_teaspoons, total_ingredients);
    let max: isize = recipes
        .iter()
        .map(|recipe| {
            (0..total_properties - 1)
                .into_iter()
                .map(|property_index| {
                    let val = (0..total_ingredients)
                        .map(|ingredient_index| {
                            data[ingredient_index][property_index]
                                * recipe[ingredient_index] as isize
                        })
                        .sum::<isize>();

                    isize::max(0, val)
                })
                .product()
        })
        .max()
        .expect("Could not find max");

    println!("Part one: {max}");

    let max_with_cal_constraint: isize = recipes
        .iter()
        .filter_map(|recipe| {
            let calories = (0..total_ingredients)
                .map(|ingredient_index| {
                    data[ingredient_index][total_properties - 1] * recipe[ingredient_index] as isize
                })
                .sum::<isize>();

            if calories != 500 {
                return None;
            }

            Some(
                (0..total_properties - 1)
                    .into_iter()
                    .map(|property_index| {
                        let val = (0..total_ingredients)
                            .map(|ingredient_index| {
                                data[ingredient_index][property_index]
                                    * recipe[ingredient_index] as isize
                            })
                            .sum::<isize>();

                        isize::max(0, val)
                    })
                    .product(),
            )
        })
        .max()
        .expect("Could not find max");

    println!("Part two: {max_with_cal_constraint}");
}

fn total_sum(total: usize, len: usize) -> Vec<Vec<usize>> {
    if len == 1 {
        return vec![vec![total]];
    }

    (0..=total)
        .flat_map(|x| {
            let mut prev_sum = total_sum(total - x, len - 1);

            prev_sum.iter_mut().for_each(|y| {
                y.push(x);
            });

            prev_sum
        })
        .collect::<Vec<_>>()
}
