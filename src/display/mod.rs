use std::{thread, time::Duration, time::Instant};
use std::io::{BufRead, BufReader};
use timeout_readwrite::TimeoutReader;
use std::io::Write;
use std::io;
use colored::*;
use crate::core::GameState; 

pub fn begin_display(initial_state: &mut GameState) {
    // let (tx, rx): (Sender<&mut GameState>, Receiver<&mut GameState>) = channel();
    let mut state = initial_state;

    while state.running {
        let current_turn = state.turn == state.id;

        if current_turn {
            clean();
            write_display(state);

            let start = Instant::now();

            let result = BufReader::new(TimeoutReader::new(io::stdin(), Duration::from_secs(12)));
            let lines = result.lines();
            for line in lines {
                let gathered = line.unwrap_or_else(|_| {
                    state.error("No valid input found.".to_string());
                    String::new()
                });

                // core.guess(gathered);

                let elapsed = start.elapsed().as_secs();
                state.time -= elapsed as i32;
                break;
            }
        } else {
            clean();
            write_display(state);
            thread::sleep(Duration::from_secs(1));
            state.time -= 1;
        }

        // core.look_for_updates();
    }
}
pub fn write_display(state: &GameState) { /* I never want to look at this method again */
    let mut dis = format!("{:-^80}\n", "rustparty");
    let current_turn = state.turn == state.id;
        
    dis.push_str(&format!("|You are Player {}!\n|\n|\
                   Prompt: \"{}\"\n|\n|", state.id.to_string().bold().blue(), state.prompt.underline()));
   
    dis.push_str(&format!("  ---{}---    ", state.lives)); 
    for player in &state.other_players {
       dis.push_str(&format!("---{}---    ", player.lives)); 
    }

    dis.push_str("\n|  ");
    dis.push_str(&("|     |    ".repeat(state.other_players.len() + 1) + "\n|"));

    let surround = if current_turn { "#".green() } else { "#".white() };
    dis.push_str(&format!("  | {}{}{} |    ", surround, state.id.to_string().bold().blue(), surround)); 
    for player in &state.other_players {
        let surround = if state.turn == player.id { "#".green() } else { "#".white() };
        dis.push_str(&format!("| {}{}{} |    ", surround, player.id.to_string().bold().cyan(), surround)); 
    }

    dis.push_str("\n|  ");
    dis.push_str(&("|     |    ".repeat(state.other_players.len() + 1) + "\n|  "));
    dis.push_str(&("-------    ".repeat(state.other_players.len() + 1) + "\n|\n|\n|"));

    dis.push_str(&format!("Time: {}        {}\n|\
                        Letters: {}\n", 
                        state.time.to_string().bold().italic().yellow(), state.current_err.bold().red(),
                        state.rem_letters));

    dis.push_str(&"-".repeat(80));
    println!("{}", dis);
    if current_turn {
        print!("{}", "| ENTER INPUT> ".green());
        io::stdout().flush().unwrap();
    }
}

pub fn clean() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}