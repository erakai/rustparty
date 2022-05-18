use serde::{Serialize, Deserialize};
use crate::connection::client::Client;
use rand::Rng;

pub const STARTING_LIVES: i32 = 2;
pub const MAX_LIVES: i32 = 3;
pub const TURN_LENGTH: i32 = 12;

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
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn to_incoming_update(update: OutgoingUpdate, current_turn: usize, prompts: &(Vec<String>, Vec<String>)) -> IncomingUpdate {
        IncomingUpdate { updated_player: Some(OutgoingUpdate::to_player_info(&update)),
                        new_used_word: Some(update.last_guess.clone()),
                        prompt: Some(create_prompt(prompts)),
                        turn: Some(current_turn as i32),
                        time: Some(TURN_LENGTH) }
    }

    pub fn to_player_info(update: &OutgoingUpdate) -> PlayerInfo {
        PlayerInfo { id: update.id, lives: update.lives, last_guess: update.last_guess.clone() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingUpdate {
    pub updated_player: Option<PlayerInfo>, 
    pub new_used_word: Option<String>,
    pub prompt: Option<String>,
    pub turn: Option<i32>,
    pub time: Option<i32>,
}

impl GameState {
    pub fn new(id: i32, client: Client, other_players: Vec<PlayerInfo>) -> GameState {
        GameState { id, 
                    running: false,
                    turn: -1,
                    lives: STARTING_LIVES, 
                    time: -1, 
                    current_err: String::from("Waiting for server host to start game..."),
                    prompt: String::new(),
                    last_guess: String::new(),
                    rem_letters: String::from("abcdefghijklmnopqrstuvwxyz"),
                    other_players,
                    used_words: Vec::new(),
                    client,
                    words: GameState::generate_words_data(), }
    }

    pub fn generate_outgoing_update(&self) -> OutgoingUpdate {
        OutgoingUpdate { id: self.id, lives: self.lives, 
                         last_guess: self.last_guess.clone(),
        }
    }

    pub fn integrate_incoming_update(&mut self, update: IncomingUpdate) {
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

    pub fn check_guess(&mut self, guess: &String){
        let valid = self.validate_word(&guess) && guess.contains(&self.prompt.to_lowercase()) && !self.used_words.contains(&guess);
        if valid {
            for &item in guess.as_bytes() {
                self.rem_letters = self.rem_letters.replace(item as char, "");
            } 
            self.check_for_extra_life();

            self.last_guess = guess.clone();
            self.used_words.push(guess.clone());
            self.time = 0;
            self.lives += 1; /* We just remove a life every time the time hits 0 */
        } else {
            self.current_err = String::from("Invalid guess! ");
            self.current_err.push_str(if guess.contains(&self.prompt) { "Already used!" } else { "Please include the prompt in a valid word!" });
        }
    }

    fn validate_word(&self, word: &String) -> bool {
        self.words.contains(word)
    }

    fn generate_words_data() -> Vec<String> {
        let word_bytes = include_bytes!("../words.txt");
        let words = String::from_utf8_lossy(word_bytes);
        words.split("\n").map(|l| l.to_string()).collect()
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
        self.error(String::from("Started!"));
        self.running = true;
        self.time = TURN_LENGTH; 
        self.turn = 0;
        self.prompt = String::from("E");
    }

    pub fn update(&mut self) -> bool {
        let mut turn_over = false;
        let mut outgoing: Option<OutgoingUpdate> = None;

        if self.time <= 0 {
            if self.turn == self.id {
                self.lives -= 1;
                outgoing = Some(self.generate_outgoing_update());
            }
            self.error(String::new());

            turn_over = true;
        } 

        let incoming = self.client.send_retrieve_update(outgoing);
        if !incoming.is_none() {
            self.integrate_incoming_update(incoming.unwrap());
        }

        turn_over
    }

    pub fn game_over(&self) -> (bool, bool) {
        let alive = self.lives > 0;
        let mut number_others_alive = 0;

        for player in &self.other_players {
            if player.lives > 0 {
                number_others_alive += 1;
            }
        }

        if alive && number_others_alive == 0 {
            return (true, true)
        }

        if !alive && number_others_alive == 1 {
            return (true, false)
        }

        return (false, false);
    }
}

pub fn generate_prompts() -> (Vec<String>, Vec<String>) {
    let prompt2_bytes = include_bytes!("../prompts2.txt");
    let prompt3_bytes = include_bytes!("../prompts3.txt");

    let prompts2 = String::from_utf8_lossy(prompt2_bytes);
    let prompts3 = String::from_utf8_lossy(prompt3_bytes);

    (prompts2.split("\n").map(|l| l.to_string()).collect(), 
     prompts3.split("\n").map(|l| l.to_string()).collect())
}

pub fn create_prompt(prompts: &(Vec<String>, Vec<String>)) -> String {
    let two_letters = rand::thread_rng().gen_bool(0.5);
    if two_letters {
        let length = prompts.0.len();
        prompts.0.get(rand::thread_rng().gen_range(0..length)).unwrap().to_string()
    } else {
        let length = prompts.1.len();
        prompts.1.get(rand::thread_rng().gen_range(0..length)).unwrap().to_string()
    }

}