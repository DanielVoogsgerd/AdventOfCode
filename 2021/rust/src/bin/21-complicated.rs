const BOARD_SIZE: usize = 10;
const MAX_SCORE: usize = 21;
const DIE_CASES: [(usize, u128); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn main() {
    let mut state  = [[[[0u128; BOARD_SIZE]; BOARD_SIZE]; MAX_SCORE]; MAX_SCORE];
    let mut current_player = 0;
    state[0][0][7][3] = 1;
    let mut player_wins = [0u128; 2];
    let mut ply = 0;
    loop {
        let mut new_state  = [[[[0u128; BOARD_SIZE]; BOARD_SIZE]; MAX_SCORE]; MAX_SCORE];
        let mut active_universes = 0u128;

        for one_score in 0..MAX_SCORE {
            for two_score in 0..MAX_SCORE {
                for one_position in 0..BOARD_SIZE {
                    for two_position in 0..BOARD_SIZE {
                        let universe_count = state[one_score][two_score][one_position][two_position];

                        let old_score = if current_player == 0 { one_score } else { two_score };
                        let old_position = if current_player == 0 { one_position } else { two_position };

                        for (move_increase, count) in DIE_CASES.iter() {
                            // This can't be the most convenient way
                            let new_position = (old_position + move_increase) % BOARD_SIZE;
                            let new_score = old_score + new_position+1;
                            if new_score >= MAX_SCORE {
                                player_wins[current_player] += count*universe_count;
                            } else {
                                active_universes += universe_count*count;
                                if current_player == 0 {
                                    new_state[new_score][two_score][new_position][two_position] += count*universe_count;
                                } else {
                                    new_state[one_score][new_score][one_position][new_position] += count*universe_count;
                                }
                            }
                        }
                    }
                }
            }
        }

        state = new_state;

        println!("Ply: {} Current player: {}, Active universes: {}, Wins: {:?}", ply, current_player, active_universes, player_wins);

        if active_universes == 0 {
            break
        }

        current_player = (current_player + 1) % 2;
        ply += 1;
    }
}
