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
    pub lives: Option<i32>,
    pub last_guess: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IncomingUpdate {
    pub other_players: Option<Vec<PlayerInfo>>, 
    pub new_used_words: Option<Vec<String>>,
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
                    current_err: String::new(),
                    prompt: String::new(),
                    last_guess: String::new(),
                    rem_letters: String::from("abcdefghijklmnopqrstuvwxyz"),
                    other_players: Vec::new(),
                    used_words: Vec::new(),
                    client, }
    }

    pub fn generate_update(&self) -> OutgoingUpdate {
        OutgoingUpdate { lives: Some(self.lives), 
                         last_guess: Some(self.last_guess.clone()),
        }
    }

    pub fn update(&mut self, update: IncomingUpdate) {
        self.other_players = update.other_players.unwrap();
        self.prompt = update.prompt.unwrap();
        self.time = update.time.unwrap();
        self.turn = update.turn.unwrap();
        
        if !update.new_used_words.is_none() {
            self.used_words.append(&mut update.new_used_words.unwrap());
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