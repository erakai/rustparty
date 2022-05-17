use serde::{Serialize, Deserialize};
use crate::connection::client::Client;

pub const STARTING_LIVES: i32 = 2;
pub const MAX_LIVES: i32 = 3;

pub struct GameState {
    pub id: i32,
    pub running: bool,
    pub turn: i32,
    pub lives: i32,
    pub time: i32,
    pub current_err: String,
    pub prompt: String,
    pub last_guess: String,
    pub rem_letters: String,
    pub other_players: Vec<PlayerInfo>,
    pub used_words: Vec<String>,
    pub client: Client,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: i32,
    pub lives: i32,
    pub last_guess: String,
}

#[derive(Serialize, Deserialize)]
pub struct OutgoingUpdate {
    pub id: i32,
    pub lives: i32,
    pub last_guess: String,
}

impl OutgoingUpdate {
    pub fn to_incoming_update(update: OutgoingUpdate, current_turn: usize, player_count: usize) -> IncomingUpdate {
        let turn = if current_turn == player_count { Some(1 as i32) } else { Some((current_turn + 1) as i32) };
        IncomingUpdate { updated_player: Some(OutgoingUpdate::to_player_info(&update)),
                        new_used_word: Some(update.last_guess.clone()),
                        prompt: Some(generate_prompt()),
                        turn,
                        time: Some(1) }
    }

    pub fn to_player_info(update: &OutgoingUpdate) -> PlayerInfo {
        PlayerInfo { id: update.id, lives: update.lives, last_guess: update.last_guess.clone() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IncomingUpdate {
    pub updated_player: Option<PlayerInfo>, 
    pub new_used_word: Option<String>,
    pub prompt: Option<String>,
    pub turn: Option<i32>,
    pub time: Option<i32>,
}

impl GameState {
    pub fn new(id: i32, client: Client) -> GameState {
        GameState { id, 
                    running: false,
                    turn: -1,
                    lives: STARTING_LIVES, 
                    time: -1, 
                    current_err: String::from("Waiting for server host to start game..."),
                    prompt: String::new(),
                    last_guess: String::new(),
                    rem_letters: String::from("abcdefghijklmnopqrstuvwxyz"),
                    other_players: Vec::new(),
                    used_words: Vec::new(),
                    client, }
    }

    pub fn generate_update(&self) -> OutgoingUpdate {
        OutgoingUpdate { id: self.id, lives: self.lives, 
                         last_guess: self.last_guess.clone(),
        }
    }

    pub fn update(&mut self, update: IncomingUpdate) {
        let updated_player = update.updated_player.unwrap();
        for player in &mut self.other_players {
            if player.id == updated_player.id {
                player.lives = updated_player.lives;
                player.last_guess = updated_player.last_guess.clone()
            }
        } 

        self.prompt = update.prompt.unwrap();
        self.time = update.time.unwrap();
        self.turn = update.turn.unwrap();
        
        if !update.new_used_word.is_none() {
            self.used_words.push(update.new_used_word.unwrap());
        }
    }

    pub fn check_guess(&mut self, guess: String) -> bool {
        //TODO: Implement checking it's an actual word
        let valid = guess.contains(&self.prompt) && !self.used_words.contains(&guess);
        if valid {
            for &item in guess.as_bytes() {
                self.rem_letters = self.rem_letters.replace(item as char, "");
            } 

            self.last_guess = guess.clone();
            self.used_words.push(guess);
            self.check_for_extra_life();
            self.time = 0;
            self.lives += 1; /* We just remove a life every time the time hits 0 */
        } else {
            self.current_err = String::from("Invalid guess! ");
            self.current_err.push_str(if guess.contains(&self.prompt) { "Already used!" } else { "Where's the prompt?" });
        }

        valid
    }

    fn check_for_extra_life(&mut self) {
        if self.rem_letters.is_empty() {
            self.current_err = "Already at max lives!".to_string();
            if self.lives < MAX_LIVES {
                self.lives += 1;
                self.current_err = "Life acquired!".to_string();
            }
            self.rem_letters = String::from("abcdefghijklmnopqrstuvwxyz");
        }
    }

    pub fn error(&mut self, desc: String) {
        self.current_err = desc;
    }

    pub fn run(&mut self) {
        self.running = true;
    }
}

fn generate_prompt() -> String {
    "lol".to_string()
}