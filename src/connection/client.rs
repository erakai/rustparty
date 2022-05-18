use std::str;
use std::net::TcpStream;
use std::io::{Read, Write};

use colored::Colorize;
use crate::{core::*, connection::server};


pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn establish(ip: String, port: usize) -> GameState {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                println!("{}", format!("Connected to {}:{}!", ip, port).green());
                println!("Waiting on the server host to begin the game..."); 

                let mut buffer = [0 as u8; server::BUFFER_SIZE];
                let size = stream.read(&mut buffer).expect("Failed to receive id.");
                let id = str::from_utf8(&buffer[0..size]).unwrap().parse().unwrap();
                println!("You are Player #{}!", id);

                buffer = [0 as u8; server::BUFFER_SIZE];
                let size = stream.read(&mut buffer).expect("Failed to receive player count.");
                let player_count = str::from_utf8(&buffer[0..size]).unwrap().parse().unwrap();
                println!("Received player count: {}.", player_count);

                let mut other_players: Vec<PlayerInfo> = Vec::new();
                for i in 0..player_count {
                    if i == id { continue };
                    other_players.push(PlayerInfo { id: i, 
                                                    lives: STARTING_LIVES, 
                                                    last_guess: String::new() })
                }

                let client = Client { stream };

                GameState::new(id, client, other_players)
            },
            Err(_) => panic!("Failed to connect to {}:{}", ip, port)
        }
    }

    pub fn send_retrieve_update(&mut self, update: Option<OutgoingUpdate>) -> IncomingUpdate {
        if !update.is_none() {
            let update = update.unwrap();
            let serialized = serde_json::to_string(&update).unwrap();
            self.stream.write(serialized.as_bytes()).expect("Failed to send OutgoingUpdate");
        }
        
        let mut buffer = [0 as u8; server::BUFFER_SIZE];
        let size = self.stream.read(&mut buffer).expect("Failed to receive IncomingUpdate");
        serde_json::from_str(str::from_utf8(&buffer[0..size]).unwrap()).unwrap()
    }
}
