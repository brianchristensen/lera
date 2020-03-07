extern crate uuid;
extern crate rand;

mod channel;
mod render;
mod server;
mod component;

use uuid::Uuid;
use std::thread;
use std::net::TcpStream;
use std::sync::mpsc::TryRecvError;
use channel::ChannelPayload;
use component::{PlayerComponent, LocationComponent};

type EntityIndex = usize;

struct GameState {
    player_components: Vec<Option<PlayerComponent>>,
    location_components: Vec<Option<LocationComponent>>,
    socket: Vec<TcpStream>,
    players: Vec<(Uuid, EntityIndex)>
}

fn main() {
    // player thread input channel - messages are all received by the main thread
    let (tx, rx) = channel::start();

    thread::spawn(move || {
        server::start(tx);
    });

    // main game loop
    loop {
        player_input_system(rx.try_recv());

        // engage other systems
    }
}

fn player_input_system(player_input: Result<ChannelPayload, TryRecvError>) {
    match player_input {
        Err(_e) => {}, // ignore disconnected socket or empty message buffer
        Ok(payload) => {
            match payload {
                ChannelPayload::Cmd(cmd) => {
                    println!("Command: {}", cmd);
                },
                ChannelPayload::Target((cmd, target)) => {
                    println!("{} does something to {}", cmd, target);
                },
                ChannelPayload::Join((id, name, _socket)) => {
                    println!("Player {} with id {} joined game", name, id);
                }
            }
        }
    }
}