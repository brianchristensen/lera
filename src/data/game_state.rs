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

pub static MAP: [[Location; MAP_X_MAX+1]; MAP_Y_MAX+1] = Location::get_map();

pub struct GameState {
    pub ecs: World,
    pub players: HashMap<Uuid, Entity>
}

impl GameState {
    // main game loop tick
    pub fn tick(&mut self, player_input: Result<ChannelPayload, TryRecvError>) {
        player_input::player_input_system(self, player_input);
        self.tick_systems();
    }

    // systems that run every tick
    fn tick_systems(&mut self) {
        let mut comm = communication::CommSystem{};
        let mut movement = movement::MoveSystem{};
        let mut aggression = ai::aggression::AggressionSystem{};

        comm.run_now(&self.ecs);
        movement.run_now(&self.ecs);
        aggression.run_now(&self.ecs);

        self.ecs.maintain();

    }

    // systems that run only on game initialization
    pub fn init(&mut self) {
        npc_manager::spawn_npcs(self);
    }

    // get player entity id from socket uuid
    pub fn get_player_eid(&mut self, id: Uuid) -> Entity {
        *self.players.get(&id).unwrap()
    }

    // get commonly used player data
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

    // direct message a player
    pub fn msg_player(&mut self, id: Uuid, msg: &str) {
        let p_entity = self.players.get(&id).unwrap();
        let p_storage = self.ecs.read_storage::<Player>();
        let p = p_storage.get(*p_entity).unwrap();
        let mut socket = p.socket.try_clone().unwrap();

        socket.write(fill(msg, TERMWIDTH).as_bytes()).unwrap();
    }

    // print the location in a specific direction
    // pub fn look_direction(&mut self, id: Uuid, direction: Direction) {

    // }

    // print a player's current location
    pub fn print_location(&mut self, id: Uuid) {
        let (eid, _name, current_loc, mut socket) = self.get_player_data(id);
        let (x, y) = current_loc.address;
        let mut exits = String::new();
        let mut player_names = String::new();
        let mut npc_names = String::new();

        if y < 9 { exits.push_str("N "); }
        if y > 0 { exits.push_str("S "); }
        if x < 9 { exits.push_str("E "); }
        if x > 0 { exits.push_str("W "); }

        let entities = self.ecs.entities();
        let players = self.ecs.read_storage::<Player>();
        let npcs = self.ecs.read_storage::<NPC>();
        let names = self.ecs.read_storage::<Name>();
        let locations = self.ecs.read_storage::<Location>();

        for (entity, _player, name, location) in (&entities, &players, &names, &locations).join() {
            if location.address == current_loc.address && eid != entity {
                player_names.push_str(format!("{} ", name.val).as_str());
            }
        };

        for (_npc, name, location) in (&npcs, &names, &locations).join() {
            if location.address == current_loc.address {
                npc_names.push_str(format!("{} ", name.val).as_str());
            }
        };

        let loc = format!("\n{}\nExits: {}\n{}{}\n",
            fill(current_loc.description, TERMWIDTH),
            exits,
            fill(player_names.as_str(), TERMWIDTH),
            fill(npc_names.as_str(), TERMWIDTH)
        );

        socket.write(loc.as_bytes()).unwrap();
    }

    pub fn add_mover(&mut self, eid: Entity, direction: Direction) {
        let mut movers = self.ecs.write_storage::<Moving>();
        movers.insert(eid, Moving { direction }).unwrap();
    }

    pub fn add_speaker(&mut self, eid: Entity, msg: &str) {
        let mut speakers = self.ecs.write_storage::<Speaking>();
        speakers.insert(eid, Speaking { msg: String::from(msg) }).unwrap();
    }

    // join a player to the game world
    pub fn join_player(&mut self, id: Uuid, name: String, socket: TcpStream) {
        let (x, y) = (0,0); // start at map location (0,0)
        let loc = MAP[x][y];
        let display_name = name.clone();

        let new_player = self.ecs
            .create_entity()
            .with(Player { socket })
            .with(Name { val: name })
            .with(loc)
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

    // remove a player from the game world
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
