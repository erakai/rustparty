mod core;
mod connection;
mod display;

use crate::connection::server::Server;
use crate::connection::client::Client;

pub fn run_server(port: usize) {
    Server::begin(port);
}

pub fn run_client(ip: String, port: usize) {
    let mut state = Client::establish(ip, port);
    state.run();
    display::begin_display(state);
}