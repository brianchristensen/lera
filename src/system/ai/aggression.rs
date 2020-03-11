use crate::data::component::*;
use crate::data::constants::*;
use crate::system::communication::pronoun;
use specs::prelude::*;
use rand::prelude::*;

pub struct AggressionSystem {}

impl<'a> System<'a> for AggressionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Aggressive>,
        WriteStorage<'a, Moving>,
        WriteStorage<'a, Speaking>
    );

    fn run(&mut self, (entities, names, aggressive, mut moving, mut speaking) : Self::SystemData) {
        let mut rng = thread_rng();
        for (eid, name, _aggressor) in (&entities, &names, &aggressive).join() {
            let mut chance = rng.gen_range(1, 101);
            // 99% chance to do nothing
            if chance <= 99 { }
            else {
                chance = rng.gen_range(1, 101);
                // in the 1% chance an npc does something, it has 2% chance to move
                if chance > 96 && chance <= 98 {
                    let move_opts = ["n", "s", "e", "w"];
                    moving.insert(eid, Moving { direction: Direction::from(String::from(move_opts[rng.gen_range(0, 4)])) }).unwrap();
                }
                // and 2% chance to emote something rude
                else if chance > 98 {
                    let emote_opts = [
                        "I'll eat your toes for breakfast!",
                        "Aaarrrrgh! Let's find that gold, it's around here somewhere...",
                        "Come on, fight me like a man!",
                        "Buuuuuuuuuurp...",
                        "You should watch your back..."
                    ];
                    let msg = format!("{} {} says, \"{}\"\n", pronoun(&name.val), name.val, emote_opts[rng.gen_range(0, 5)]);
                    speaking.insert(eid, Speaking { msg }).unwrap();
                }
            }
        };
    }
}
