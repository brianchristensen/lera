use crate::data::game_state::MAP;
use crate::data::component::*;
use crate::data::constants::*;
use crate::system::communication::pronoun;
use specs::prelude::*;
use textwrap::fill;
use std::io::Write;

pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, NPC>,
        WriteStorage<'a, Moving>,
        WriteStorage<'a, Location>
    );

    fn run(&mut self, (entities, names, players, npcs, mut moving, mut locations) : Self::SystemData) {
        let mut new_locs: Vec<(Entity, Location)> = vec![];

        for (entity, m, name, loc) in (&entities, &moving, &names, &locations).join() {
            let (can_move, new_addr, to, from) = bounds_check(loc.address, &m.direction);

            if can_move {
                for (eid, player, p_loc) in (&entities, &players, &locations).join() {
                    // print to players from old room
                    if p_loc.address == loc.address && eid != entity {
                        let f_msg = match players.get(entity) {
                            None => format!("{} {} exits to the {}\n", pronoun(&name.val), name.val, to),
                            Some(_p) => format!("{} exits to the {}\n", name.val, to)
                        };
                        let mut txs = player.socket.try_clone().unwrap();
                        txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
                    }
                    // print to players in new room
                    else if p_loc.address == new_addr && eid != entity {
                        let f_msg = match players.get(entity) {
                            None => format!("{} {} enters from the {}\n", pronoun(&name.val), name.val, from),
                            Some(_p) => format!("{} enters from the {}\n", name.val, to)
                        };
                        let mut txs = player.socket.try_clone().unwrap();
                        txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
                    }
                }
                let (x, y) = new_addr;
                new_locs.push((entity, MAP[x][y]))
            } else {
                let is_player = players.get(entity);
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

        // remove old location component and add new location component
        for mover in new_locs {
            let (e, l) = mover;
            locations.remove(e);
            locations.insert(e, l).unwrap();
            // if entity is a player, print the new location data
            let is_player = players.get(e);
            match is_player {
                None => {},
                Some(p) => {
                    let (x, y) = l.address;
                    let mut exits = String::new();
                    let mut player_names = String::new();
                    let mut npc_names = String::new();

                    if y < MAP_Y_MAX { exits.push_str("N "); }
                    if y > MAP_Y_MIN { exits.push_str("S "); }
                    if x < MAP_X_MAX { exits.push_str("E "); }
                    if x > MAP_X_MIN { exits.push_str("W "); }

                    for (entity, _player, name, location) in (&entities, &players, &names, &locations).join() {
                        if location.address == l.address && e != entity {
                            player_names.push_str(format!("{} ", name.val).as_str());
                        }
                    };

                    for (_npc, name, location) in (&npcs, &names, &locations).join() {
                        if location.address == l.address {
                            npc_names.push_str(format!("{} ", name.val).as_str());
                        }
                    };

                    let loc = format!("\n{}\nExits: {}\n{}{}\n",
                        fill(l.description, TERMWIDTH),
                        exits,
                        fill(player_names.as_str(), TERMWIDTH),
                        fill(npc_names.as_str(), TERMWIDTH)
                    );

                    let mut txs = p.socket.try_clone().unwrap();
                    txs.write(loc.as_bytes()).unwrap();
                }
            }
        }

        // remove moving components
        for e in (&entities, &moving).join().map(|(e, _s)| e).collect::<Vec<Entity>>() {
            moving.remove(e);
        };
    }
}

pub fn bounds_check(curr_addr: (usize, usize), direction: &Direction) -> (bool, (usize, usize), &'static str, &'static str) {
    let (x, y) = curr_addr;
    let mut new_addr: (usize, usize) = curr_addr;
    match direction {
        Direction::N => {
            if y < MAP_Y_MAX {
                new_addr = (x, y+1);
                (true, new_addr, "north", "south")
            } else {
                (false, new_addr, "north", "")
            }
        },
        Direction::S => {
            if y > MAP_Y_MIN {
                new_addr = (x, y-1);
                (true, new_addr, "south", "north")
            } else {
                (false, new_addr, "south", "")
            }
        },
        Direction::E => {
            if x < MAP_X_MAX {
                new_addr = (x+1, y);
                (true, new_addr, "east", "west")
            } else {
                (false, new_addr, "east", "")
            }
        },
        Direction::W => {
            if x > MAP_X_MIN {
                new_addr = (x-1, y);
                (true, new_addr, "west", "east")
            } else {
                (false, new_addr, "west", "")
            }
        }
    }
}
