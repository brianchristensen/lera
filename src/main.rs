extern crate uuid;
extern crate rand;

mod server;
mod world;
mod player;
mod location;

use uuid::Uuid;
use world::World;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::<(Uuid, String)>();
    let world = Arc::new(Mutex::new(World::new()));

    let internal_world = world.clone();
    thread::spawn(move || {
        world::listen(internal_world, rx);
    });

    let user_world = world.clone();
    server::start(user_world, tx);
}
