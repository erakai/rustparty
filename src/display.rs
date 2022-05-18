use std::{thread, time::Duration, time::Instant};
use std::io::{BufRead, BufReader, Write};
use timeout_readwrite::TimeoutReader;
use std::{io, process};
use colored::*;
use crate::core::GameState;

pub fn begin_display(initial_state: GameState) {
    let mut state = initial_state;
    loop {
        let current_turn = state.turn == state.id;

        clean();
        write_display(&mut state);

        let (over, winner) = state.game_over();
        if over {
            write_game_over(winner);
            process::exit(1);
        }

        if current_turn  && state.lives > 0 {

            let start = Instant::now();

            let result = BufReader::new(TimeoutReader::new(io::stdin(), Duration::from_secs(1)));
            let lines = result.lines();
            for line in lines {
                let gathered = line.unwrap_or(String::new());

                state.check_guess(&gathered);

                let elapsed = start.elapsed().as_secs();
                state.time -= elapsed as i32;
                break;
            }
        } else if state.lives == 0 {
            state.time = 0;
        } else {
            thread::sleep(Duration::from_secs(1));
            state.time -= 1;
        }

        state.update();


        if !state.running {
            break;
        }
    }
}

pub fn write_game_over(won: bool) {
    println!("{}", format!("\nYOU {}!", if won { "WON"} else { "LOST" }).bold());
}

pub fn write_display(state: &GameState) { /* I never want to look at this method again */
    let mut dis = format!("{:-^80}\n", "rustparty");
    let current_turn = state.turn == state.id;
        
    dis.push_str(&format!("You are Player {}! Lives: {}\n\nTurn: {}\n\
                   Prompt: \"{}\"\n\n", state.id.to_string().bold(), state.lives.to_string().green(), 
                   state.turn.to_string().bold().green(), state.prompt.underline()));
   
    dis.push_str(&format!("  ---{}---    ", state.lives.to_string().green())); 
    for player in &state.other_players {
       dis.push_str(&format!("---{}---    ", player.lives.to_string().green())); 
    }

    dis.push_str("\n  ");
    dis.push_str(&("|     |    ".repeat(state.other_players.len() + 1) + "\n"));

    let middle = if current_turn { format!("#{}#", state.id.to_string().bold()).bright_green() } else { format!("#{}#", state.id.to_string().bold()).white() };
    dis.push_str(&format!("  | {} |    ", middle)); 
    for player in &state.other_players {
        let middle = if state.turn == player.id { format!("#{}#", player.id.to_string().bold()).bright_green() } 
            else { format!("#{}#", player.id.to_string().bold()).white() };
        dis.push_str(&format!("| {} |    ", middle)); 
    }

    dis.push_str("\n  ");
    dis.push_str(&("|     |    ".repeat(state.other_players.len() + 1) + "\n  "));
    dis.push_str(&("-------    ".repeat(state.other_players.len() + 1) + "\n"));

    let mut guess = state.last_guess.as_str();
    if guess.len() > 5 {
        guess = &state.last_guess[0..5];
    }

    if current_turn {
        guess = "";
    }

    dis.push_str(&format!("   {}{}     ", guess, " ".repeat(5 - guess.len())));
    for player in &state.other_players {
        guess = player.last_guess.as_str();
        if guess.len() > 5 {
            guess = &player.last_guess[0..5];
        }

        if player.id == state.turn {
            guess = "";
        }
        dis.push_str(&format!(" {}{}     ", guess, " ".repeat(5 - guess.len())));
    }


    dis.push_str(&format!("\n\n\nTime: {}        {}\n\
                        Letters: {}\n", 
                        state.time.to_string().bold().italic().yellow(), state.current_err.bold().red(),
                        state.rem_letters));

    dis.push_str(&"-".repeat(80));

    if current_turn {
        dis.push_str(&format!("{}", "\nType in your input and press enter to submit or clear.\n".to_string().green()));
    }
    print!("{}", dis);
    io::stdout().flush().unwrap();

}

pub fn clean() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}