use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, rc::Rc};

#[derive(Clone, Debug)]
struct Effect {
    r#type: EffectType,
    timer: usize,
    cost: usize,
    damage: usize,
    armor: usize,
    mana_boost: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum EffectType {
    Shield,
    Poison,
    Recharge,
}

impl Effect {
    fn new(r#type: EffectType) -> Self {
        match r#type {
            EffectType::Shield => Self {
                r#type,
                timer: 6,
                cost: 113,
                damage: 0,
                armor: 7,
                mana_boost: 0,
            },
            EffectType::Poison => Self {
                r#type,
                timer: 6,
                cost: 173,
                damage: 3,
                armor: 0,
                mana_boost: 0,
            },
            EffectType::Recharge => Self {
                r#type,
                timer: 5,
                cost: 229,
                damage: 0,
                armor: 0,
                mana_boost: 101,
            },
        }
    }

    fn next(&self) -> Option<Self> {
        (self.timer > 1).then(|| Self {
            r#type: self.r#type,
            timer: self.timer - 1,
            cost: self.cost,
            damage: self.damage,
            armor: self.armor,
            mana_boost: self.mana_boost,
        })
    }
}

#[derive(Clone)]
struct Spell {
    r#type: SpellType,
    cost: usize,
    damage: usize,
    health: usize,
}

#[derive(Clone)]
enum SpellType {
    MagicMissile,
    Drain,
}

impl Spell {
    fn new(r#type: SpellType) -> Self {
        match r#type {
            SpellType::MagicMissile => Self {
                r#type,
                cost: 53,
                damage: 4,
                health: 0,
            },
            SpellType::Drain => Self {
                r#type,
                cost: 73,
                damage: 2,
                health: 2,
            },
        }
    }
}

enum Cast<'a> {
    Spell(&'a Spell),
    Effect(&'a Effect),
}

#[derive(Clone, Debug)]
enum Player {
    HumanPlayer(HumanPlayer),
    Boss(Boss),
}

#[derive(Clone, Debug)]
struct HumanPlayer {
    health: usize,
    mana: usize,
    active_effects: Vec<Effect>,
}

impl HumanPlayer {
    fn new(health: usize, mana: usize) -> Self {
        Self {
            health,
            mana,
            active_effects: vec![],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Boss {
    health: usize,
    damage: usize,
}

impl Boss {
    fn new(health: usize, damage: usize) -> Self {
        Self { health, damage }
    }
}

struct Game {
    available_spells: Rc<Vec<Spell>>,
    available_effects: Rc<Vec<Effect>>,

    players: [Player; 2],
    turn: usize,
}

impl Game {
    fn new(player1: Player, player2: Player) -> Self {
        let spells = vec![
            Spell::new(SpellType::MagicMissile),
            Spell::new(SpellType::Drain),
        ];

        let effects = vec![
            Effect::new(EffectType::Shield),
            Effect::new(EffectType::Poison),
            Effect::new(EffectType::Recharge),
        ];

        Self {
            available_spells: Rc::new(spells),
            available_effects: Rc::new(effects),
            turn: 0,
            players: [player1, player2],
        }
    }

    /// Makes a copy of the game, but creates a new reference to the RC, since they are "Frozen".
    fn clone(&self) -> Self {
        Self {
            available_spells: Rc::clone(&self.available_spells),
            available_effects: Rc::clone(&self.available_effects),
            turn: self.turn,
            players: self.players.clone(),
        }
    }

    fn get_available_casts(&self) -> impl Iterator<Item = Cast> + '_ {
        let effects = self.available_effects.iter().map(|x| Cast::Effect(x));
        let spells = self.available_spells.iter().map(|x| Cast::Spell(x));
        effects.chain(spells)
    }
}

#[derive(PartialEq)]
enum GameType {
    Normal,
    Hard,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spent_mana = self.spent_mana;
        let turn = self.game.turn;
        let p1 = &self.game.players[0];
        let p2 = &self.game.players[1];

        write!(
            f,
            "Spent mana: {spent_mana}; Turn: {turn}\nPlayer 1: {p1:#?}\nPlayer 2: {p2:#?}\n"
        )
    }
}

struct GameState {
    game: Game,
    spent_mana: usize,
}

impl Eq for GameState {}
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.spent_mana == other.spent_mana
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.spent_mana.cmp(&other.spent_mana))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.spent_mana.cmp(&other.spent_mana)
    }
}

fn main() {
    let player = Player::HumanPlayer(HumanPlayer::new(50, 500));
    let boss = Player::Boss(Boss::new(71, 10));

    let initial_game = Game::new(player, boss);
    let cost = play_game(initial_game, GameType::Normal);
    println!("{cost}");

    let player = Player::HumanPlayer(HumanPlayer::new(50, 500));
    let boss = Player::Boss(Boss::new(71, 10));
    let initial_game = Game::new(player, boss);
    let cost = play_game(initial_game, GameType::Hard);
    println!("{cost}");
}

fn play_game(initial_game: Game, game_type: GameType) -> usize {
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(GameState {
        game: initial_game,
        spent_mana: 0,
    }));

    while let Some(current_state) = heap.pop() {
        let current_state = current_state.0;
        let turn = current_state.game.turn;
        let attacking_player = &current_state.game.players[turn % 2];
        let defending_player = &current_state.game.players[(turn + 1) % 2];

        let (att_damage, att_armor, mut new_att_player) = match attacking_player {
            Player::HumanPlayer(human) => {
                let mut new_effects = vec![];
                let mut total_armor = 0;
                let mut total_damage = 0;
                let mut mana_gain = 0;
                for effect in &human.active_effects {
                    if let Some(new_effect) = effect.next() {
                        new_effects.push(new_effect);
                    }

                    total_damage += effect.damage;
                    total_armor += effect.armor;
                    mana_gain += effect.mana_boost;
                }

                let mut new_human = human.clone();
                new_human.mana += mana_gain;
                new_human.active_effects = new_effects;
                (total_damage, total_armor, Player::HumanPlayer(new_human))
            }
            Player::Boss(boss) => (0, 0, Player::Boss(*boss)),
        };

        let (def_damage, def_armor, mut new_def_player) = match defending_player {
            Player::HumanPlayer(human) => {
                let mut new_effects = vec![];
                let mut total_armor = 0;
                let mut total_damage = 0;
                let mut mana_gain = 0;
                for effect in &human.active_effects {
                    if let Some(new_effect) = effect.next() {
                        new_effects.push(new_effect);
                    }

                    total_damage += effect.damage;
                    total_armor += effect.armor;
                    mana_gain += effect.mana_boost;
                }
                let mut new_human = human.clone();
                new_human.mana += mana_gain;
                new_human.active_effects = new_effects;
                (total_damage, total_armor, Player::HumanPlayer(new_human))
            }
            Player::Boss(boss) => (0, 0, Player::Boss(*boss)),
        };

        // Can be fixed by wrapping it in another struct with a common health attr.
        // This is about passive damage from effects. Spells will be done seperately, but is behind a branch
        if att_damage > 0 {
            match &mut new_def_player {
                Player::HumanPlayer(human) => {
                    let rel_damage = usize::max(att_damage, 1 + def_armor) - def_armor;
                    if rel_damage >= human.health {
                        continue;
                    }
                    human.health -= rel_damage;
                }
                Player::Boss(boss) => {
                    let rel_damage = usize::max(att_damage, 1 + def_armor) - def_armor;
                    if rel_damage >= boss.health {
                        return current_state.spent_mana;
                    }
                    boss.health -= rel_damage;
                }
            };
        }

        if def_damage > 0 {
            // Attacking player can still get damage from effects
            match &mut new_att_player {
                Player::HumanPlayer(human) => {
                    let rel_damage = usize::max(def_damage, 1 + att_armor) - att_armor;
                    if rel_damage >= human.health {
                        continue;
                    }
                    human.health -= rel_damage;
                }
                Player::Boss(boss) => {
                    let rel_damage = usize::max(def_damage, 1 + att_armor) - att_armor;
                    if rel_damage >= boss.health {
                        return current_state.spent_mana;
                    }
                    boss.health -= rel_damage;
                }
            };
        }

        if game_type == GameType::Hard {
            if let Player::HumanPlayer(human) = &mut new_att_player {
                if human.health <= 1 {
                    continue;
                }
                human.health -= 1;
            }
        }

        match new_att_player {
            Player::HumanPlayer(new_att_human) => {
                for cast in current_state.game.get_available_casts() {
                    let mut spent_mana = current_state.spent_mana;
                    let mut new_att_human = new_att_human.clone();
                    let new_def_player = match cast {
                        Cast::Spell(spell) => {
                            if new_att_human.mana < spell.cost {
                                continue;
                            }
                            new_att_human.mana -= spell.cost;
                            spent_mana += spell.cost;
                            new_att_human.health += spell.health;
                            match &new_def_player {
                                Player::HumanPlayer(def_human) => {
                                    let mut new_def_human = def_human.clone();
                                    let rel_damage = spell.damage - def_armor;
                                    if rel_damage >= new_def_human.health {
                                        continue;
                                    }
                                    new_def_human.health = new_def_human.health - rel_damage;
                                    Player::HumanPlayer(new_def_human)
                                }
                                Player::Boss(boss) => {
                                    let mut new_boss = boss.clone();
                                    if spell.damage >= boss.health {
                                        return spent_mana;
                                    }
                                    new_boss.health -= spell.damage;
                                    Player::Boss(new_boss)
                                }
                            }
                        }
                        Cast::Effect(effect) => {
                            if new_att_human
                                .active_effects
                                .iter()
                                .any(|active_effect| active_effect.r#type == effect.r#type)
                            {
                                continue;
                            }
                            if new_att_human.mana < effect.cost {
                                continue;
                            }
                            new_att_human.mana -= effect.cost;
                            spent_mana += effect.cost;

                            new_att_human.active_effects.push(effect.clone());
                            // Might be better with Rc
                            new_def_player.clone()
                        }
                    };

                    let new_att_player = Player::HumanPlayer(new_att_human);

                    let players = if current_state.game.turn % 2 == 0 {
                        [new_att_player, new_def_player]
                    } else {
                        [new_def_player, new_att_player]
                    };

                    let new_game = Game {
                        available_spells: Rc::clone(&current_state.game.available_spells),
                        available_effects: Rc::clone(&current_state.game.available_effects),
                        players,
                        turn: current_state.game.turn + 1,
                    };
                    let new_game_state = GameState {
                        game: new_game,
                        spent_mana,
                    };
                    heap.push(Reverse(new_game_state));
                }
            }
            // Boss already did his damage
            Player::Boss(boss) => {
                let rel_damage = usize::max(boss.damage, def_armor + 1) - def_armor;

                match &mut new_def_player {
                    Player::HumanPlayer(h) => {
                        if h.health < rel_damage {
                            continue;
                        }
                        h.health -= rel_damage
                    }
                    Player::Boss(b) => {
                        if b.health < rel_damage {
                            return current_state.spent_mana;
                        }
                        b.health -= rel_damage;
                    }
                }

                // We need to place the players back in the same order, maybe this is not the best idea.
                let players = if current_state.game.turn % 2 == 0 {
                    [new_att_player, new_def_player]
                } else {
                    [new_def_player, new_att_player]
                };
                let new_game = Game {
                    available_spells: Rc::clone(&current_state.game.available_spells),
                    available_effects: Rc::clone(&current_state.game.available_effects),
                    players,
                    turn: current_state.game.turn + 1,
                };
                let new_game_state = GameState {
                    game: new_game,
                    spent_mana: current_state.spent_mana,
                };
                heap.push(Reverse(new_game_state));
            }
        };
    }

    panic!("No gamestates lead anywhere")
}
