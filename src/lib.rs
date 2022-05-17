mod core;
mod connection;
mod display;

use crate::core::*;
use crate::connection::client;
use crate::connection::server;

pub fn run_test() {
    /* 
    let mut test_state: core::GameState = core::GameState::new(1);
    let vec = vec![core::PlayerInfo {id: 2, lives: 2, last_guess: String::new() },
                   core::PlayerInfo {id: 3 , lives: 2, last_guess: String::new() },
                   core::PlayerInfo {id: 4 , lives: 2, last_guess: String::new() },
                   core::PlayerInfo {id: 5 , lives: 2, last_guess: String::new() },
                   core::PlayerInfo {id: 6 , lives: 2, last_guess: String::new() },
                   core::PlayerInfo {id: 7, lives: 2, last_guess: String::new() }]; 
    let update = core::IncomingUpdate {
       other_players: Some(vec),
       new_used_words: None,
       prompt: Some(String::from("sh")),
       turn: Some(1),
       time: Some(12), 
    };

    test_state.update(update);
    
    test_state.run();
    display::begin_display(&mut test_state);
    */
}

pub fn run_game(is_client: bool, domain: String, port: usize) {
    // do some kind of connection thingy
}

pub fn guessed(state: &mut GameState, guess: String) -> bool {
    state.check_guess(guess)
}

pub fn update(state: &mut GameState) {
    if state.time <= 0 {
        state.lives -= 1;
        // state.client.send(state.generate_update());
    } 
}