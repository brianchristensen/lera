#[macro_use] extern crate specs_derive;
extern crate uuid;
extern crate rand;

mod channel;
mod render;
mod server;
mod component;
mod map;

use specs::prelude::*;
use rand::prelude::*;
use std::thread;
use std::io::Write;
use channel::ChannelPayload;
use component::*;

struct State {
    ecs: World,
    map: Vec<Location>
}

fn main() {
    // init game state
    let mut gs = State {
        ecs: World::new(),
        map: Location::gen_map()
    };
    gs.ecs.register::<Player>();
    gs.ecs.register::<Location>();

    // player thread input channel - messages are all received by the main thread
    let (tx, rx) = channel::start();

    thread::spawn(move || {
        server::start(tx);
    });

    let mut rng = rand::thread_rng();
    // main game loop
    loop {
        let player_input = rx.try_recv();
        match player_input {
            Err(_e) => {}, // ignore disconnected socket or empty message buffer
            Ok(payload) => {
                match payload {
                    ChannelPayload::Cmd((id, cmd)) => {
                        match cmd.as_str() {
                            "look" => {},
                            _ => {}
                        }
                    },
                    ChannelPayload::Target((id, cmd, target)) => {
                        println!("{} does something to {}", cmd, target);
                    },
                    ChannelPayload::Join((id, name, mut socket)) => {
                        println!("Player {} with id {} joined game", name, id);
                        let loc = gs.map[rng.gen_range(0, 101)];
                        let s = socket.try_clone().unwrap();
                        gs.ecs
                            .create_entity()
                            .with(Player { id, name, socket: s })
                            .with(loc)
                            .build();
                        socket.write(loc.description.as_bytes()).unwrap();
                        socket.write("\n".as_bytes()).unwrap();
                    }
                }
            }
        }

        // engage other systems
    }
}
