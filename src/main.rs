use std::{thread, time::Duration};

mod core;
mod display;

fn main() {
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
}
