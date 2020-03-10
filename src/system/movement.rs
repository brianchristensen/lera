use crate::data::component::*;
use crate::data::constants::*;
use specs::prelude::*;
use textwrap::fill;
use std::io::Write;

pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Moving>,
        WriteStorage<'a, Location>
    );

    fn run(&mut self, (entities, names, players, mut moving, mut locations) : Self::SystemData) {
        let mut new_locs: Vec<(Entity, Location)> = vec![];
        for (entity, m, name, loc) in (&entities, &moving, &names, &locations).join() {
            for (eid, player, p_loc) in (&entities, &players, &locations).join() {
                // print to players from old room
                if p_loc.address == loc.address && eid != entity {
                  let f_msg = format!("{} exits to the {}\n", name.val, m.to);
                  let mut txs = player.socket.try_clone().unwrap();
                  txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
                }
                // print to players from new room
                else if p_loc.address == m.loc.address && eid != entity {
                    let f_msg = format!("{} enters from the {}\n", name.val, m.from);
                    let mut txs = player.socket.try_clone().unwrap();
                    txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
                }
            }
            new_locs.push((entity, m.loc));
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

                    if y < 9 { exits.push_str("N "); }
                    if y > 0 { exits.push_str("S "); }
                    if x < 9 { exits.push_str("E "); }
                    if x > 0 { exits.push_str("W "); }

                    for (entity, name, location) in (&entities, &names, &locations).join() {
                        if location.address == l.address && e != entity {
                            player_names.push_str(format!("{} ", name.val).as_str());
                        }
                    };

                    let loc = format!("\n{}\nExits: {}\n{}\n\n",
                        fill(l.description, TERMWIDTH),
                        exits,
                        fill(player_names.as_str(), TERMWIDTH)
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
