use itertools::{chain, iproduct, Itertools};

fn main() {
    let boss = (109, 8, 2);
    let player_health = 100;

    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    let rings = [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let weapon_choice = weapons.iter().combinations(1);

    let armor_choice = chain!(
        std::iter::once(vec![&(0, 0, 0)]),
        armor.iter().combinations(1),
    );

    let ring_choice = chain!(
        std::iter::once(vec![&(0, 0, 0)]),
        rings.iter().combinations(1),
        rings.iter().combinations(2),
    );

    let mut outfit = iproduct!(weapon_choice, armor_choice, ring_choice)
        .map(|(weapon, armor, ring)| {
            chain!(weapon, armor, ring).fold((0, 0, 0), |acc, cur| {
                (acc.0 + cur.0, acc.1 + cur.1, acc.2 + cur.2)
            })
        })
        .collect::<Vec<_>>();

    // Sort by cost
    outfit.sort_by_key(|x| x.0);

    let least_cost_winning_outfit = outfit.iter().find(|(_, damage, armor)| {
        let boss_damage = i32::max(boss.1 - armor, 1);
        let player_damage = i32::max(damage - boss.2, 1);

        let boss_turns = player_health / boss_damage
            + if (player_health % boss_damage) == 0 {
                0
            } else {
                1
            };
        let player_turns =
            boss.0 / player_damage + if (boss.0 % player_damage) == 0 { 0 } else { 1 };

        player_turns <= boss_turns
    });

    println!("{least_cost_winning_outfit:?}");

    let highest_cost_losing_outfit = outfit.iter().rev().find(|(_, damage, armor)| {
        let boss_damage = i32::max(boss.1 - armor, 1);
        let player_damage = i32::max(damage - boss.2, 1);

        let boss_turns = player_health / boss_damage
            + if (player_health % boss_damage) == 0 {
                0
            } else {
                1
            };
        let player_turns =
            boss.0 / player_damage + if (boss.0 % player_damage) == 0 { 0 } else { 1 };

        player_turns > boss_turns
    });

    println!("{highest_cost_losing_outfit:?}");
}
