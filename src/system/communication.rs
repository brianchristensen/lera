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
                    let f_msg = format!("{}: {}\n", name.val, speak.msg);
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
