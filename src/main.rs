#[macro_use] extern crate specs_derive;
extern crate uuid;
extern crate rand;
extern crate textwrap;

mod title;
mod server;
mod data;
mod system;

use specs::prelude::*;
use data::game_state::*;
use data::component::*;
use data::channel;
use std::thread;
use std::collections::HashMap;

fn main() {
    // init game state
    let mut gs = GameState {
        ecs: World::new(),
        players: HashMap::new(),
        map: Location::gen_map()
    };
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Location>();

    // player thread input channel - messages are all received by the main thread from the socket server
    let (tx, rx) = channel::start();

    // start listening for player connections
    thread::spawn(move || {
        server::start(tx);
    });

    // main game loop
    loop {
        let player_input = rx.try_recv();
        gs.tick(player_input);
    }
}
