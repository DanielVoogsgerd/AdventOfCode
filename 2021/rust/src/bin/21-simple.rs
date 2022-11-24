const BOARD_SIZE: u32 = 10;
const WINNING_SCORE: u32 = 1000;

fn main() {
    let board_size = 10;
    let mut die = DiracDie::new(100);
    let mut players = vec![
        Player::new(BOARD_SIZE, WINNING_SCORE, 8),
        Player::new(BOARD_SIZE, WINNING_SCORE, 4),
    ];

    let mut winner_index: Option<usize> = None;
    loop {
        for (player_index, mut player) in players.iter_mut().enumerate() {
            player.turn(&mut die);
            if player.has_won() {
                winner_index = Some(player_index);
                break
            }
        }

        if winner_index.is_some() {
            break
        }
    }

    println!("Player {} won!", winner_index.unwrap());
    for (player_index, player) in players.iter().enumerate() {
        println!("Player {} score: {}!", player_index, player.score);
    }
    let loser_index = if winner_index.unwrap() == 1 {0} else {1};
    println!("Total die rolls: {}", die.total_rolls);
    println!("Result part 1: {}", die.total_rolls * players[loser_index].score);
}

struct DiracDie {
    sides: u32,
    state: u32,
    total_rolls: u32
}

impl DiracDie {
    fn new(sides: u32) -> Self {
       Self { sides, state: 0, total_rolls: 0 }
    }

    fn roll(&mut self) -> u32 {
        self.total_rolls += 1;
        let state = self.state + 1;
        self.state = (self.state + 1) % self.sides;
        return state;
    }
}

struct Player {
    score: u32,
    winning_score: u32,
    position: u32,
    board_size: u32,
}

impl Player {
    fn new(board_size: u32, winning_score: u32, starting_position: u32) -> Self {
        Self {
            score: 0,
            winning_score: winning_score,
            position: starting_position,
            board_size,
        }
    }

    fn turn(&mut self, die: &mut DiracDie) {
        let result = (0..3).map(|_| { die.roll() }).map(|x| {
            // println!("Roll {}", x);
            x
        }).sum();
        // println!("Result: {}", result);
        self.make_move(result);
        println!("Score: {}", self.score);
    }

    fn make_move(&mut self, move_size: u32) {
        self.position = ((self.position + move_size -1) % self.board_size) +1;
        self.score += self.position
    }

    fn has_won(&self) -> bool {
        self.score >= self.winning_score
    }
}
