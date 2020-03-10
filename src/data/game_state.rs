use crate::channel::ChannelPayload;
use crate::data::component::*;
use crate::data::constants::*;
use crate::system::*;
use specs::prelude::*;
use uuid::Uuid;
use textwrap::fill;
use std::net::TcpStream;
use std::io::Write;
use std::sync::mpsc::TryRecvError;
use std::collections::HashMap;

pub struct GameState {
    pub ecs: World,
    pub map: HashMap<(usize, usize), Location>,
    pub players: HashMap<Uuid, Entity>
}

impl GameState {
    pub fn tick(&mut self, player_input: Result<ChannelPayload, TryRecvError>) {
        player_input::player_input_system(self, player_input);
        self.run_systems();
    }

    fn run_systems(&mut self) {
        let mut comm = communication::CommSystem{};
        let mut movement = movement::MoveSystem{};
        comm.run_now(&self.ecs);
        movement.run_now(&self.ecs);
        self.ecs.maintain();
    }

    pub fn get_player_eid(&mut self, id: Uuid) -> Entity {
        *self.players.get(&id).unwrap()
    }

    pub fn get_player_data(&mut self, id: Uuid) -> (Entity, String/*name*/, Location, TcpStream) {
        let p_entity = self.players.get(&id).unwrap();
        let players = self.ecs.read_storage::<Player>();
        let locations = self.ecs.read_storage::<Location>();
        let names = self.ecs.read_storage::<Name>();
        let p = players.get(*p_entity).unwrap();
        let loc = locations.get(*p_entity).unwrap();
        let p_name = names.get(*p_entity).unwrap().val.as_str();
        let socket = p.socket.try_clone().unwrap();

        (*p_entity, String::from(p_name), *loc, socket)
    }

    pub fn msg_player(&mut self, id: Uuid, msg: &str) {
        let p_entity = self.players.get(&id).unwrap();
        let p_storage = self.ecs.read_storage::<Player>();
        let p = p_storage.get(*p_entity).unwrap();
        let mut socket = p.socket.try_clone().unwrap();

        socket.write(fill(msg, TERMWIDTH).as_bytes()).unwrap();
    }

    pub fn print_location(&mut self, id: Uuid) {
        let (eid, _name, current_loc, mut socket) = self.get_player_data(id);
        let (x, y) = current_loc.address;
        let mut exits = String::new();
        let mut entity_names = String::new();

        if y < 9 { exits.push_str("N "); }
        if y > 0 { exits.push_str("S "); }
        if x < 9 { exits.push_str("E "); }
        if x > 0 { exits.push_str("W "); }

        let entities = self.ecs.entities();
        let names = self.ecs.read_storage::<Name>();
        let locations = self.ecs.read_storage::<Location>();

        for (entity, name, location) in (&entities, &names, &locations).join() {
            if location.address == current_loc.address && eid != entity {
                entity_names.push_str(format!("{} ", name.val).as_str());
            }
        };

        let loc = format!("\n{}\nExits: {}\n{}\n",
            fill(current_loc.description, TERMWIDTH),
            exits,
            fill(entity_names.as_str(), TERMWIDTH)
        );

        socket.write(loc.as_bytes()).unwrap();
    }

    pub fn add_mover(&mut self, eid: Entity, direction: Direction) {
        let locations = self.ecs.read_storage::<Location>();
        let (x, y) = locations.get(eid).unwrap().address;
        let mut new_addr: (usize, usize) = (x, y);

        let (can_move, to, from): (bool, &str, &str) = match direction {
            Direction::N => {
                if y < MAP_Y_MAX {
                    new_addr = (x, y+1);
                    (true, "north", "south")
                } else {
                    (false, "north", "")
                }
            },
            Direction::S => {
                if y > MAP_Y_MIN {
                    new_addr = (x, y-1);
                    (true, "south", "north")
                } else {
                    (false, "south", "")
                }
            },
            Direction::E => {
                if x < MAP_X_MAX {
                    new_addr = (x+1, y);
                    (true, "east", "west")
                } else {
                    (false, "east", "")
                }
            },
            Direction::W => {
                if x > MAP_X_MIN {
                    new_addr = (x-1, y);
                    (true, "west", "east")
                } else {
                    (false, "west", "")
                }
            }
        };
        if can_move {
            let mut movers = self.ecs.write_storage::<Moving>();
            movers.insert(eid, Moving { loc: *self.map.get(&new_addr).unwrap(), to: String::from(to), from: String::from(from) }).unwrap();
        } else {
            // if there is no location in the specified direction and entity is a player
            // notify the player that there is no such exit
            let players = self.ecs.read_storage::<Player>();
            let is_player = players.get(eid);
            match is_player {
                None => {},
                Some(p) => {
                    let f_msg = format!("There is no exit to the {}\n", to);
                    let mut txs = p.socket.try_clone().unwrap();
                    txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
                }
            }
        }
    }

    pub fn add_speaker(&mut self, eid: Entity, msg: &str) {
        let mut speakers = self.ecs.write_storage::<Speaking>();
        speakers.insert(eid, Speaking { msg: String::from(msg) }).unwrap();
    }

    pub fn join_player(&mut self, id: Uuid, name: String, socket: TcpStream) {
        let addr: (usize, usize) = (0,0); // start at map location (0,0)
        let loc = self.map.get(&addr).unwrap();
        let display_name = name.clone();

        let new_player = self.ecs
            .create_entity()
            .with(Player { socket })
            .with(Name { val: name })
            .with(*loc)
            .build();
        self.players.insert(id, new_player);

        let entities = self.ecs.entities();
        let players = self.ecs.read_storage::<Player>();
        let locations = self.ecs.read_storage::<Location>();

        for (eid, player, p_loc) in (&entities, &players, &locations).join() {
            // print to players from old room
            if p_loc.address == loc.address && eid != new_player {
              let f_msg = format!("{} has arrived in Lera\n", display_name);
              let mut txs = player.socket.try_clone().unwrap();
              txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
            }
        }
    }

    pub fn remove_player(&mut self, id: Uuid) {
        let (eid, name, current_loc, _socket) = self.get_player_data(id);
        self.ecs.delete_entity(eid).unwrap();

        let entities = self.ecs.entities();
        let players = self.ecs.read_storage::<Player>();
        let locations = self.ecs.read_storage::<Location>();

        for (entity, player, p_loc) in (&entities, &players, &locations).join() {
            // print to players from the room the client disconnected from
            if p_loc.address == current_loc.address && eid != entity {
              let f_msg = format!("{} has left the land of Lera\n", name);
              let mut txs = player.socket.try_clone().unwrap();
              txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
            }
        }
    }
}
