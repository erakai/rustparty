use std::str;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

use colored::Colorize;

use crate::core::OutgoingUpdate;

pub const BUFFER_SIZE: usize = 200;
pub const TIMEOUT_MILLIS: u64 = 250;


struct Worker;

impl Worker {
    fn new(id: usize, mut stream: TcpStream, sender: Arc<Mutex<mpsc::Sender<String>>>,
            receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Worker {
        thread::spawn(move ||  {
            stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT_MILLIS))).expect("Setting timeout failed");
            stream.write(&id.to_string().as_bytes()).expect("Failed to properly send id");

            let player_count: usize = receiver.lock().unwrap().recv().unwrap().parse().unwrap();    
            let mut current_turn = 1;
            
            stream.write("RUN".to_string().as_bytes()).expect("Failed to send RUN to clients");
            let mut buffer = [0 as u8; BUFFER_SIZE]; // 200 byte buffer?

            loop {
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let received = str::from_utf8(&buffer[0..size]).unwrap();
                        println!("\nThread #{} received data: {}", id, received);

                        sender.lock().unwrap().send(received.to_string()).unwrap();
                        
                        current_turn += 1;
                    },
                    Err(_) => {}
                }
                let received = receiver.lock().unwrap().recv_timeout(Duration::from_millis(TIMEOUT_MILLIS)).unwrap();     

                let deserialized: OutgoingUpdate = serde_json::from_str(&received).unwrap();
                let new_update = OutgoingUpdate::to_incoming_update(deserialized, current_turn, player_count);

                let string = serde_json::to_string(&new_update).unwrap();
                println!("\nID: {} Sending data: {}", id, string);
                stream.write(&string.as_bytes()).expect("Failed to write correctly");

                current_turn += 1;
            }
        });
        Worker { }
    }
}

pub struct Server {
    threads: Vec<Worker>,
    sender: Arc<Mutex<mpsc::Sender<String>>>,
    receiver: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl Server {
    pub fn begin(port: usize) -> Server {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        let mut server = Server::new(7);

        println!("{}", String::from("Listening for servers!").green());
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            server.add_client(stream);
        }

        server
    }

    pub fn new(size: usize) -> Server {
        assert!(size > 0);
        let threads = Vec::new();

        let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver));

        Server { threads, sender, receiver }
    }

    pub fn run(&self) {
        self.sender.lock().unwrap().send(self.threads.len().to_string()).unwrap();
    }

    fn add_client(&mut self, stream: TcpStream) {
        let worker = Worker::new(self.threads.len(), stream, Arc::clone(&self.sender), Arc::clone(&self.receiver));
        self.threads.push(worker);
    }

}