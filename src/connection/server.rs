use std::{str, io};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread;
use crossbeam_channel::{unbounded, Sender, Receiver};

use colored::Colorize;

use crate::core;

pub const BUFFER_SIZE: usize = 400;
pub const TIMEOUT_MILLIS: u64 = 250;


struct Worker;

impl Worker {
    fn new(id: usize, mut stream: TcpStream, sender: Sender<String>,
            receiver: Receiver<String>) -> Worker {
        thread::spawn(move ||  {
            stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT_MILLIS / 2))).expect("Setting timeout failed");
            stream.write(&id.to_string().as_bytes()).expect("Failed to properly send id");

            let prompts = core::generate_prompts();
            let player_count: usize = receiver.recv().unwrap().parse().unwrap();    
            let mut current_turn = 0;
            
            println!("Thread #{} is sending player count of {}.", id, player_count);
            stream.write(player_count.to_string().as_bytes()).expect("Failed to send Player Count to clients");
            let mut buffer = [0 as u8; BUFFER_SIZE]; // 200 byte buffer?

            loop {
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let packet = str::from_utf8(&buffer[0..size]).unwrap().to_string();
                        println!("\nThread #{} (current_turn: {}) received data: {}", id, current_turn, packet);

                        current_turn += 1;
                        if current_turn == player_count {
                            current_turn = 0;
                        }

                        let deserialized: core::OutgoingUpdate = serde_json::from_str(&packet).unwrap();
                        let new_update = core::OutgoingUpdate::to_incoming_update(deserialized, current_turn, &prompts);
                        let string = serde_json::to_string(&new_update).unwrap();

                        for _ in 0..player_count {
                            sender.send(string.clone()).unwrap();
                        }
                    },
                    Err(_) => {}
                }
                let string = receiver.recv_timeout(Duration::from_millis(TIMEOUT_MILLIS)).unwrap_or("None".to_string());      
                let received = if string == "None" { None } else { Some(string) };

                if !received.is_none() {
                    let received = received.unwrap();

                    let deserialized: core::IncomingUpdate = serde_json::from_str(&received).unwrap();
                    current_turn = deserialized.turn.unwrap() as usize;

                    println!("  ID: {} is sending data: {}", id, received);
                    stream.write(&received.as_bytes()).expect("Failed to write correctly");
                }
            }
        });
        Worker { }
    }
}

pub struct Server {
    threads: Vec<Worker>,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Server {
    pub fn begin(port: usize) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        let mut server = Server::new(7);

        println!("{}", String::from("Waiting for players!").green());
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Player {} connected from {}!", server.threads.len(), stream.local_addr().unwrap().ip());   
            server.add_client(stream);

            if server.threads.len() >= 2 {
                let mut input = String::new();

                print!("\nWould you like to begin the game (y/n)? > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Failed to receive input");

                match input.trim().to_lowercase().as_str() {
                    "y" => break,
                    "n" => println!("Waiting for more players...\n"),
                    _ => println!("Assuming that's a no...\n"),
                }
            }

        }

        server.run();

        let mut input = String::new();
        println!("{}", format!("Press enter to close the server at any point.\n").bold().red());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to receive input");
    }

    pub fn new(size: usize) -> Server {
        assert!(size > 0);
        let threads = Vec::new();

        let (sender, receiver) = unbounded();

        Server { threads, sender, receiver }
    }

    pub fn run(&self) {
        println!("{}", format!("Starting server with {} players...", self.threads.len()).green());
        for _ in 0..self.threads.len() {
            self.sender.send(self.threads.len().to_string()).unwrap();
        }
    }

    fn add_client(&mut self, stream: TcpStream) {
        let worker = Worker::new(self.threads.len(), stream, self.sender.clone(), self.receiver.clone());
        self.threads.push(worker);
    }

}