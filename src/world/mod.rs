mod title;

use crate::player::Player;
use crate::location::Location;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::net::{TcpStream};
use std::io::{Write};
use std::collections::HashMap;

pub struct World {
    pub player_list: HashMap<Uuid, Player>,
    pub world_map: Vec<Vec<Location>>
}

impl World {
    pub fn new() -> World {
        let w = World {
            player_list: HashMap::new(),
            world_map: Location::gen_map()
        };
        return w;
    }
}

// listen for user commands *world thread
pub fn listen(world: Arc<Mutex<World>>, rx: Receiver<(Uuid, String)>) {
    // TODO: init world resources
    loop {
        let (id, raw_cmd) = rx.recv().unwrap();
        let fmt_cmd = raw_cmd.replace("\n", "");
        let cmd = fmt_cmd.as_str().split(' ').collect::<Vec<&str>>();
        let p = &world.lock().unwrap().player_list[&id];

        match cmd[0].to_lowercase().as_str() {
            "name" => {
                if p.name == "" {
                    //p.name = String::from(cmd[1]);
                    p.write(format!("You have set your name to: {}", cmd[1]));
                } else {
                    p.write(String::from("You can only set your name once."));
                }
            },
            "look" => {
                p.look();
            },
            _ =>  {
                p.write(format!("Unrecognized command: {}", cmd[0]));
            }
        }

        p.write(String::from("\n> "));
    }
}

// join a new player to the world *player thread
pub fn join(world: Arc<Mutex<World>>, id: Uuid, mut socket: TcpStream) {
    socket.write(title::print()).unwrap();
    socket.write("Create a player by typing \"name {player name}\"\n".as_bytes()).unwrap();
    socket.write("See other commands by typing \"help\"\n\n".as_bytes()).unwrap();
    socket.write("> ".as_bytes()).unwrap();

    let p = Player::new(id, String::from(""), world.clone(), socket);
    world.lock().unwrap().player_list.insert(id, p);
}
