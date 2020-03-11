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
use std::{thread, time};
use std::collections::HashMap;

fn main() {
    // create game state struct
    let mut gs = GameState {
        ecs: World::new(),
        players: HashMap::new()
    };
    // register entity components
    gs.ecs.register::<Player>();
    gs.ecs.register::<NPC>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Location>();
    gs.ecs.register::<Speaking>();
    gs.ecs.register::<Moving>();
    gs.ecs.register::<Aggressive>();
    // init game state
    gs.init();

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
        // 16 tps
        thread::sleep(time::Duration::from_millis(16));
    }
}
