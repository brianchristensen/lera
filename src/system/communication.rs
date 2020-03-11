use crate::data::component::*;
use crate::data::constants::TERMWIDTH;
use specs::prelude::*;
use textwrap::fill;
use std::io::Write;

pub struct CommSystem {}

impl<'a> System<'a> for CommSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Speaking>,
        ReadStorage<'a, Location>
    );

    fn run(&mut self, (entities, names, players, mut speaking, locations) : Self::SystemData) {
        for (entity, speak, name, loc) in (&entities, &speaking, &names, &locations).join() {
            for (eid, player, p_loc) in (&entities, &players, &locations).join() {
                if p_loc.address == loc.address && eid != entity {
                    let is_player = players.get(entity);
                    let mut f_msg = speak.msg.clone();
                    match is_player {
                        None => {}, // npc messages are displayed as is
                        Some(_p) => { f_msg = format!("{}: {}\n", name.val, speak.msg); } // player messages are displayed chat style
                    };
                    let mut txs = player.socket.try_clone().unwrap();
                    txs.write(fill(f_msg.as_str(), TERMWIDTH).as_bytes()).unwrap();
                }
            }
        };
        // remove speaking components
        for e in (&entities, &speaking).join().map(|(e, _s)| e).collect::<Vec<Entity>>() {
            speaking.remove(e);
        };
    }
}

pub fn pronoun(name: &String) -> &'static str {
    match name.chars().nth(0).unwrap() {
        'a' | 'A' | 'e' | 'E' | 'o' | 'O' | 'i' | 'I' | 'u' | 'U' => "An",
        _ => "A"
    }
}
