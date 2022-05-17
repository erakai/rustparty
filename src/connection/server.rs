use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, Arc, Mutex};

use crate::core::OutgoingUpdate;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, stream: TcpStream, sender: Arc<Mutex<mpsc::Sender<String>>>,
            receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Worker {
        let thread = thread::spawn(move ||  {
                
        });
        Worker { id, thread }
    }
}

pub struct ThreadHandler {
    threads: Vec<Worker>,
    sender: Arc<Mutex<mpsc::Sender<String>>>,
    receiver: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl ThreadHandler {
    pub fn new(size: usize) -> ThreadHandler {
        assert!(size > 0);
        let threads = Vec::new();

        let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver));

        ThreadHandler { threads, sender, receiver }
    }

    pub fn client(&mut self, stream: TcpStream) {
        let worker = Worker::new(self.threads.len(), stream, Arc::clone(&self.sender), Arc::clone(&self.receiver));
        self.threads.push(worker);
    }

}

pub struct Server;

impl Server {
    fn run(port: i32) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        let mut pool = ThreadHandler::new(7);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.client(stream);
        }
    }

}

