
fn main() {
    // target area: x=124..174, y=-123..-86
    let target = ((124, 174), (-123, -86));
    // let target = ((20, 30), (-10, -5));
    let environment = Environment {
        source: (0, 0),
        target
    };

    println!("Answer part 1: {}", environment.highest_point());
    println!("Answer part 2: {}", environment.get_valid_shots().len());
}

struct Environment {
    source: (i32, i32),
    target: ((i32, i32), (i32, i32))
}

impl Environment {
    fn highest_point(&self) -> i32 {
        let dy = self.source.1 - self.target.1.0 - 1;
        (dy*(dy + 1))/2
    }

    fn on_target(&self, x: i32, y: i32) -> bool {
        x >= self.target.0.0 &&
        x <= self.target.0.1 && 
        y >= self.target.1.0 &&
        y <= self.target.1.1
    }

    fn valid_shot(&self, initial_velocity: (i32, i32)) -> bool {
        let mut i = 0;
        let mut cur_x_velocity = initial_velocity.0;
        let mut cur_y_velocity = initial_velocity.1;
        let mut x_pos = 0;
        let mut y_pos = 0;

        // Optimisation when the target is below up, yet we're shooting up.
        // if initial_velocity.1 > 0 && self.target.1.1 < 0 {
        //     // We do not care for all points above ourselves.
        //     // That is n-1 points on the way up, n-1 points on the way down and one maximum.
        //     i = 2*initial_velocity.1 - 1;
        //     cur_x_velocity = i32::max(initial_velocity.0-i, 0);
        //     cur_y_velocity = -1 * initial_velocity.1;
        //     if initial_velocity.0 > i {
        //         x_pos = ((initial_velocity.0 * (initial_velocity.0 + 1)) / 2) - ((i*(i+1)) / 2);
        //     } else {
        //         x_pos = (initial_velocity.0 * (initial_velocity.0 + 1)) / 2
        //     }
        // }

        while x_pos <= self.target.0.1 && y_pos >= self.target.1.0 {
            x_pos += cur_x_velocity;
            y_pos += cur_y_velocity;

            if cur_x_velocity > 0 {
                cur_x_velocity -= 1
            }

            cur_y_velocity -= 1;

            // Check if target is hit
            if self.on_target(x_pos, y_pos) {
                return true;
            }
        }

        false
    }

    fn get_valid_shots(&self) -> Vec<(i32, i32)>{
        let mut valid_shots = Vec::new();
        for x in (inverse_triangle_number(self.target.0.0 as u32) as i32)..=(self.target.0.1+1) {
            for y in (self.target.1.0)..(-self.target.1.0) {
                if self.valid_shot(((x as i32, y as i32))) {
                    valid_shots.push((x, y));
                }
            }
        }

        valid_shots
    }
}

fn inverse_triangle_number(x: u32) -> u32 {
    (f32::sqrt((8*x+1) as f32) as u32 - 1) / 2
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{Environment, inverse_triangle_number};

    #[test]
    fn test_inverse_triangle_number() {
        assert_eq!(10, inverse_triangle_number(55));
    }

    #[test]
    fn test_part_two_example_input() {
        let valid_shots = [
            (23,-10), (25,-9), (27,-5), (29,-6), (22,-6), (21,-7), (9,0), (27,-7), (24,-5), (25,-7),
            (26,-6), (25,-5), (6,8), (11,-2), (20,-5), (29,-10), (6,3), (28,-7), (8,0), (30,-6),
            (29,-8), (20,-10), (6,7), (6,4), (6,1), (14,-4), (21,-6), (26,-10), (7,-1), (7,7),
            (8,-1), (21,-9), (6,2), (20,-7), (30,-10), (14,-3), (20,-8), (13,-2), (7,3), (28,-8),
            (29,-9), (15,-3), (22,-5), (26,-8), (25,-8), (25,-6), (15,-4), (9,-2), (15,-2), (12,-2),
            (28,-9), (12,-3), (24,-6), (23,-7), (25,-10), (7,8), (11,-3), (26,-7), (7,1), (23,-9),
            (6,0), (22,-10), (27,-6), (8,1), (22,-8), (13,-4), (7,6), (28,-6), (11,-4), (12,-4),
            (26,-9), (7,4), (24,-10), (23,-8), (30,-8), (7,0), (9,-1), (10,-1), (26,-5), (22,-9),
            (6,5), (7,5), (23,-6), (28,-10), (10,-2), (11,-1), (20,-9), (14,-2), (29,-7), (13,-3),
            (23,-5), (24,-8), (27,-9), (30,-7), (28,-5), (21,-10), (7,9), (6,6), (21,-5), (27,-10),
            (7,2), (30,-9), (21,-8), (22,-7), (24,-9), (20,-6), (6,9), (29,-5), (8,-2), (27,-8),
            (30,-5), (24, -7) ];

        let target = ((20, 30), (-10, -5));
        let environment = Environment {
            source: (0, 0),
            target
        };

        let mut collection = valid_shots.iter().collect::<HashSet<_>>();
        for shot in environment.get_valid_shots() {
            assert!(valid_shots.contains(&shot));
            collection.remove(&shot);
        }

        println!("Missing shots: {:?}", collection);
        assert!(collection.is_empty());
    }
}