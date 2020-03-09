use crate::data::component::Location;
use crate::channel::ChannelPayload;
use crate::system::player_input::player_input_system;
use crate::data::component::*;
use specs::prelude::*;
use uuid::Uuid;
use textwrap::fill;
use std::net::TcpStream;
use std::io::Write;
use std::sync::mpsc::TryRecvError;
use std::collections::HashMap;

const TERMWIDTH: usize = 80;

pub struct GameState {
    pub ecs: World,
    pub map: Vec<Location>,
    pub players: HashMap<Uuid, Entity>
}

impl GameState {
    pub fn tick(&mut self, player_input: Result<ChannelPayload, TryRecvError>) {
        player_input_system(self, player_input);
    }

    pub fn get_player_data(&mut self, id: Uuid) -> (Entity, String/*name*/, Location, TcpStream) {
        let p_entity = self.players.get(&id).unwrap();
        let players = self.ecs.read_storage::<Player>();
        let locations = self.ecs.read_storage::<Location>();
        let names = self.ecs.read_storage::<Name>();
        let p = players.get(*p_entity).unwrap();
        let loc = locations.get(*p_entity).unwrap();
        let p_name = names.get(*p_entity).unwrap().name.as_str();
        let socket = p.socket.try_clone().unwrap();

        (*p_entity, String::from(p_name), *loc, socket)
    }

    pub fn dm_player(&mut self, id: Uuid, msg: &str) {
        let p_entity = self.players.get(&id).unwrap();
        let p_storage = self.ecs.read_storage::<Player>();
        let p = p_storage.get(*p_entity).unwrap();
        let mut socket = p.socket.try_clone().unwrap();

        socket.write(fill(msg, TERMWIDTH).as_bytes()).unwrap();
    }

    pub fn dm_location(&mut self, id: Uuid, msg: &str) {
        let (eid, name, current_loc, _socket) = self.get_player_data(id);
        let entities = self.ecs.entities();
        let players = self.ecs.read_storage::<Player>();
        let locations = self.ecs.read_storage::<Location>();
        let f_msg = format!("{}: {}\n", name, msg);

        for (entity, player, location) in (&entities, &players, &locations).join() {
            if location.address == current_loc.address && eid != entity {
                let mut txs = player.socket.try_clone().unwrap();
                txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
            }
        };
    }

    pub fn msg_location(&mut self, eid: Entity, msg: &str) {
        let entities = self.ecs.entities();
        let players = self.ecs.read_storage::<Player>();
        let locations = self.ecs.read_storage::<Location>();
        let current_loc = locations.get(eid).unwrap();
        let names = self.ecs.read_storage::<Name>();
        let name = names.get(eid).unwrap().name.as_str();

        let f_msg = format!("{}: {}\n", name, msg);

        for (entity, player, location) in (&entities, &players, &locations).join() {
            if location.address == current_loc.address && eid != entity {
                let mut txs = player.socket.try_clone().unwrap();
                txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
            }
        };
    }

    pub fn print_location(&mut self, id: Uuid) {
        let (eid, _name, current_loc, mut socket) = self.get_player_data(id);
        let (x, y) = current_loc.address;
        let mut exits = String::new();
        let mut player_names = String::new();

        if y < 9 { exits.push_str("N "); }
        if y > 0 { exits.push_str("S "); }
        if x < 9 { exits.push_str("E "); }
        if x > 0 { exits.push_str("W "); }

        let entities = self.ecs.entities();
        let players = self.ecs.read_storage::<Player>();
        let names = self.ecs.read_storage::<Name>();
        let locations = self.ecs.read_storage::<Location>();

        for (entity, _player, name_cont, location) in (&entities, &players, &names, &locations).join() {
            if location.address == current_loc.address && eid != entity {
                player_names.push_str(format!("{} ", name_cont.name).as_str());
            }
        };

        let loc = format!("\n{}\nExits: {}\n{}\n\n",
            fill(current_loc.description, TERMWIDTH),
            exits,
            fill(player_names.as_str(), TERMWIDTH)
        );

        socket.write(loc.as_bytes()).unwrap();
    }

    pub fn move_player(&mut self, id: Uuid, cmd: String) {
        let (eid, _name, current_loc, mut socket) = self.get_player_data(id);
        let (x, y) = current_loc.address;

        if  y == 9 && cmd == "n" ||
            y == 0 && cmd == "s" ||
            x == 9 && cmd == "e" ||
            x == 0 && cmd == "w" { socket.write("There is no exit in that direction\n".as_bytes()).unwrap(); }
    }

    pub fn join_player(&mut self, id: Uuid, name: String, socket: TcpStream) {
        let loc = self.map[0];

        let player = self.ecs
            .create_entity()
            .with(Player { socket })
            .with(Name { name })
            .with(loc)
            .build();

        self.players.insert(id, player);
    }
}
